#![allow(clippy::upper_case_acronyms)]

use crate::*;

use email_wallet_utils::generate_claim_input;
use ethers::types::Address;

#[derive(Debug, Clone)]
pub struct Claim {
    pub tx_hash: String,
    pub id: U256,
    pub email_address: String,
    pub random: String,
    pub commit: String,
    pub expiry_time: i64,
    pub is_fund: bool,
    pub is_announced: bool,
    pub is_seen: bool,
}

#[named]
pub async fn claim_unclaims(mut claim: Claim) -> Result<EmailWalletEvent> {
    let mut need_creation = false;
    let is_seen = claim.is_seen;
    if DB
        .get_claims_by_id(&claim.id)
        .await?
        .into_iter()
        .filter(|c: &Claim| c.is_fund == claim.is_fund)
        .collect::<Vec<_>>()
        .len()
        == 0
    {
        claim.is_seen = true;
        DB.insert_claim(&claim).await?;
        let psi_client = PSIClient::new(
            claim.email_address.to_string(),
            claim.tx_hash.clone(),
            claim.id,
            claim.is_fund,
        )
        .await?;
        need_creation = psi_client.check_and_reveal().await?;
    }
    if need_creation && !DB.contains_user(&claim.email_address).await.unwrap() {
        let account_key = AccountKey::new(rand::thread_rng());
        let account_key_str = field2hex(&account_key.0);
        let psi_point = compute_psi_point(
            CIRCUITS_DIR_PATH.get().unwrap(),
            &claim.email_address,
            &account_key_str,
        )
        .await?;
        let wallet_salt = WalletSalt::new(
            &PaddedEmailAddr::from_email_addr(&claim.email_address),
            account_key,
        )?;
        let tx_hash = CLIENT.register_psi_point(&psi_point, &wallet_salt).await?;
        info!(LOG, "register psi point tx hash: {}", tx_hash; "func" => function_name!());
        let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;

        DB.insert_user(
            &claim.email_address,
            &account_key_str,
            "",
            false,
            &format!("0x{}", hex::encode(wallet_addr.as_bytes())),
        )
        .await?;
        return Ok(EmailWalletEvent::AccountNotCreated {
            email_addr: claim.email_address,
            account_key,
            is_first: true,
            tx_hash,
        });
    }
    let account_key_str = if let Some(key) = DB.get_account_key(&claim.email_address).await? {
        key
    } else {
        return Ok(EmailWalletEvent::NoOp);
    };
    let is_account_created = CLIENT
        .check_if_account_created_by_account_key(&claim.email_address, &account_key_str)
        .await?;
    if !is_seen && !is_account_created {
        return Ok(EmailWalletEvent::AccountNotCreated {
            email_addr: claim.email_address,
            account_key: AccountKey(hex2field(&account_key_str)?),
            is_first: false,
            tx_hash: "".to_string(),
        });
        // return Err(anyhow!("Account not created"));
    } else if !is_account_created {
        return Err(anyhow!("Account not created"));
    }
    let account_key = AccountKey(hex2field(&account_key_str)?);
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&claim.email_address);
    let wallet_salt = WalletSalt::new(&padded_email_addr, account_key)?;
    let now = now();

    let (unclaimed_fund, unclaimed_state) = if claim.is_fund {
        let unclaimed_fund = CLIENT.query_unclaimed_fund(claim.id).await?;
        if unclaimed_fund.expiry_time.as_u64() < u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim expired"));
        }
        (Some(unclaimed_fund), None)
    } else {
        let unclaimed_state = CLIENT.query_unclaimed_state(claim.id).await?;
        if unclaimed_state.expiry_time.as_u64() < u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim expired"));
        }
        if claim.is_announced
            && is_installed_extension(unclaimed_state.extension_addr, &wallet_salt).await?
        {
            return Err(anyhow!(
                "Unclaimed state anounces the email address but its extension is not installed."
            ));
        }
        (None, Some(unclaimed_state))
    };

    let input = generate_claim_input(
        &claim.email_address,
        &claim.random,
        &field2hex(&account_key.0),
    )
    .await?;
    let (proof, pub_signals) =
        generate_proof(&input, "claim", PROVER_ADDRESS.get().unwrap()).await?;
    info!(LOG, "original commit {}", claim.commit; "func" => function_name!());
    info!(LOG, "original randomness {}", claim.random; "func" => function_name!());
    info!(LOG, "commit in pub signals: {}", pub_signals[0]; "func" => function_name!());
    let data = ClaimInput {
        id: claim.id,
        recipient_wallet_salt: u256_to_bytes32(&pub_signals[1]),
        is_fund: claim.is_fund,
        proof,
    };
    let tx_hash = CLIENT.claim(data).await?;
    DB.delete_claim(&claim.id, claim.is_fund).await?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&wallet_salt.0).await?;
    Ok(EmailWalletEvent::Claimed {
        unclaimed_fund,
        unclaimed_state,
        email_addr: claim.email_address,
        is_fund: claim.is_fund,
        is_announced: claim.is_announced,
        recipient_account_key: account_key,
        tx_hash,
    })
}

async fn is_installed_extension(extension_addr: Address, wallet_salt: &WalletSalt) -> Result<bool> {
    let subject_templates = CLIENT
        .query_subject_templates_of_extension(extension_addr)
        .await?;
    let command = subject_templates[0][0].as_str();
    let installed_extension = CLIENT
        .query_user_extension_for_command(wallet_salt, command)
        .await?;
    Ok(installed_extension == extension_addr)
}
