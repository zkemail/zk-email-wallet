use anyhow::{anyhow, Result};

use crate::{
    error, handle_email, handle_email_event, render_html, trace, wallet::EphemeralTx, EmailMessage,
    EmailWalletEvent,
};
use ethers::types::{Address, Bytes, U256};
use relayer_utils::{
    converters::{field2hex, hex2field},
    cryptos::{AccountCode, AccountSalt, PaddedEmailAddr},
    ParsedEmail, LOG,
};

use crate::{CHAIN_RPC_EXPLORER, CLIENT, DB};
use hex::encode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use serde_json::Value;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct NFTTransferRequest {
    pub email_addr: String,
    pub nft_id: u64,
    pub nft_addr: String,
    pub recipient_addr: String,
    pub is_recipient_email: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub email_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendRequest {
    pub email_addr: String,
    pub amount: Number,
    pub token_id: String,
    pub recipient_addr: String,
    pub is_recipient_email: bool,
}

#[derive(Serialize, Deserialize)]
pub struct IsAccountCreatedRequest {
    pub email_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWalletAddress {
    pub email_addr: String,
    pub account_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecoverAccountCode {
    pub email_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailAddrCommitRequest {
    pub email_address: String,
    pub random: String,
}

#[derive(Serialize, Deserialize)]
pub struct UnclaimRequest {
    pub email_address: String,
    pub random: String,
    pub expiry_time: i64,
    pub is_fund: bool,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRegistrationRequest {
    pub email_address: String,
    pub account_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRegistrationResponse {
    pub account_code: String,
    pub wallet_addr: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatResponse {
    pub onboarding_tokens_distributed: u32,
    pub onboarding_tokens_left: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SafeRequest {
    pub wallet_addr: String,
    pub safe_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupRequest {
    pub email_addr: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SigninRequest {
    pub email_addr: String,
    pub username: String,
    pub nonce: String,
    pub expiry_time: Option<Number>,
    pub token_allowances: Option<Vec<(Number, String)>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterEpheAddrRequest {
    pub wallet_addr: String,
    pub username: String,
    pub ephe_addr: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteEphemeralTxRequest {
    pub wallet_addr: String,
    pub tx_nonce: String,
    pub ephe_addr: String,
    pub ephe_addr_nonce: String,
    pub target: String,
    pub eth_value: String,
    pub data: String,
    pub token_amount: String,
    pub signature: String,
}

pub async fn nft_transfer_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<NFTTransferRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let nft_addr = Address::from_str(&request.nft_addr)?;
    let nft_name = CLIENT.query_nft_name_of_address(nft_addr).await?;
    let subject = format!(
        "NFT Send {} of {} to {}",
        request.nft_id, nft_name, request.recipient_addr
    );
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let body_plain = format!(
        "Hi {}! Please reply to this email to send {} your NFT: ID {} of {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr,  request.recipient_addr, request.nft_id, nft_name, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let nft_uri = CLIENT
        .query_erc721_token_uri_of_token(nft_addr, U256::from(request.nft_id))
        .await?;
    let json_uri: Value = serde_json::from_str(
        &String::from_utf8(
            base64::decode(nft_uri.split(",").nth(1).expect("Invalid base64 string"))
                .expect("Failed to decode base64 string"),
        )
        .expect("Invalid UTF-8 sequence"),
    )
    .expect("Failed to parse JSON");
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "nftName": nft_name, "nftID": request.nft_id, "recipientAddr": request.recipient_addr, "walletAddr": wallet_addr, "img":"cid:0", "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "img": json_uri["image"].as_str().unwrap_or_default()});
    let body_html = render_html("nft_transfer.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: Some(vec![]),
    };
    Ok((request_id, email))
}

pub async fn create_account_api_fn(payload: String) -> Result<(String, EmailMessage)> {
    let request = serde_json::from_str::<CreateAccountRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let email_addr = request.email_addr;
    let account_code_str = DB.get_account_code(&email_addr).await?;
    if account_code_str.is_none() {
        let account_code = AccountCode::new(rand::thread_rng());
        let invitation_code_hex = &field2hex(&account_code.0)[2..];
        let subject = format!(
            "Email Wallet Account Creation. Code {}",
            invitation_code_hex
        );
        let render_data = serde_json::json!({"userEmailAddr": email_addr.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("account_creation.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: email_addr.clone(),
            body_plain: "To create your email wallet, reply to this email.".to_string(),
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        Ok((field2hex(&account_code.0), email))
    } else {
        let subject = "Email Wallet Error: Account Already Exists".to_string();
        let error_msg =
            "Your wallet is already created. Please use the login page instead.".to_string();
        // TODO: Get user's account address
        let account_code = AccountCode(hex2field(&account_code_str.clone().unwrap())?);
        let account_salt =
            AccountSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_code)?;
        let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
        let render_data = serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap(), "accountCode": account_code_str.unwrap(), "walletAddr": wallet_addr});
        let body_html = render_html("account_already_exist.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        Ok(("0x".to_string(), email))
    }
}

pub async fn is_account_created_api_fn(payload: String) -> Result<bool> {
    let request = serde_json::from_str::<IsAccountCreatedRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn send_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<SendRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let subject = format!(
        "Send {} {} to {}",
        request.amount, request.token_id, request.recipient_addr
    );
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let body_plain = format!(
        "Hi {}! Please reply to this email to send {} {} to {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr, request.amount, request.token_id, request.recipient_addr, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "originalSubject": subject, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("send_request.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

pub async fn get_wallet_address_api_fn(payload: String) -> Result<String> {
    let request = serde_json::from_str::<GetWalletAddress>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        return Err(anyhow!(
            "Account key not found for email address: {}",
            request.email_addr
        ));
    }
    let account_code = AccountCode(hex2field(&request.account_code)?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    Ok("0x".to_string() + &encode(&wallet_addr.0))
}

pub async fn recover_account_code_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<RecoverAccountCode>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let email_addr = request.email_addr;
    let account_code_str = DB.get_account_code(&email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_code_hex = &field2hex(&account_code.0)[2..];
    let account_salt =
        AccountSalt::new(&PaddedEmailAddr::from_email_addr(&email_addr), account_code)?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let subject = "Email Wallet Account Login".to_string();
    let render_data = serde_json::json!({"userEmailAddr": email_addr, "accountCode": account_code_hex, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("account_recovery.html", render_data).await?;
    let email = EmailMessage {
        subject,
        to: email_addr.clone(),
        body_plain: format!("Hi {}! Your account key is {}, keep it in a safe space.\nYour wallet address: {}/address/{}.", email_addr, account_code_hex, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr),
        body_html,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

pub async fn add_safe_owner_api_fn(payload: String) -> Result<()> {
    let request = serde_json::from_str::<SafeRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;

    let is_wallet_addr_email_wallet = DB.is_wallet_addr_exist(&request.wallet_addr).await?;

    if is_wallet_addr_email_wallet {
        DB.add_user_with_safe(&request.wallet_addr, &request.safe_addr)
            .await?;
    }

    Ok(())
}

pub async fn delete_safe_owner_api_fn(payload: String) -> Result<()> {
    let request = serde_json::from_str::<SafeRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;

    let is_wallet_addr_email_wallet = DB.is_wallet_addr_exist(&request.wallet_addr).await?;

    if is_wallet_addr_email_wallet {
        DB.remove_safe_from_user(&request.wallet_addr, &request.safe_addr)
            .await?;
    }

    Ok(())
}

pub async fn receive_email_api_fn(email: String) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await.unwrap();
    let from_addr = parsed_email.get_from_addr().unwrap();
    tokio::spawn(async move {
        match handle_email_event(EmailWalletEvent::Ack {
            email_addr: from_addr.clone(),
            subject: parsed_email.get_subject_all().unwrap_or_default(),
            original_message_id: parsed_email.get_message_id().ok(),
        })
        .await
        {
            Ok(_) => {
                trace!(LOG, "Ack email event sent");
            }
            Err(e) => {
                error!(LOG, "Error handling email event: {:?}", e);
            }
        }
        match handle_email(email.clone()).await {
            Ok(event) => match handle_email_event(event).await {
                Ok(_) => {}
                Err(e) => {
                    error!(LOG, "Error handling email event: {:?}", e);
                }
            },
            Err(e) => {
                error!(LOG, "Error handling email: {:?}", e);
                match handle_email_event(EmailWalletEvent::Error {
                    email_addr: from_addr,
                    error: e.to_string(),
                })
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!(LOG, "Error handling email event: {:?}", e);
                    }
                }
            }
        }
    });
    Ok(())
}

pub async fn signup_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<SignupRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let subject = format!("Sign-up {}", request.username);
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let body_plain = format!(
        "Hi {}! Please reply to this email to sign-up {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr, request.username, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "originalSubject": subject, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("send_request.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

pub async fn signin_api_fn(payload: String) -> Result<(u64, EmailMessage)> {
    let request_id = rand::thread_rng().gen();
    let request = serde_json::from_str::<SigninRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let mut subject_words = vec!["Sign-in".to_string()];
    subject_words.push(request.username.clone());
    subject_words.push("on device".to_string());
    subject_words.push(request.nonce.clone());
    if let Some(expiry_time) = request.expiry_time {
        subject_words.push("until timestamp".to_string());
        subject_words.push(expiry_time.to_string());
    }
    if let Some(token_allowances) = request.token_allowances {
        subject_words.push("for".to_string());
        for (amount, token_name) in token_allowances {
            subject_words.push(format!("{} {}", amount.to_string(), token_name));
        }
    }
    let subject = subject_words.join(" ");
    let account_code_str = DB.get_account_code(&request.email_addr).await?;
    if account_code_str.is_none() {
        let subject = "Email Wallet Error: Account Not Found".to_string();
        let error_msg =
            "Your wallet is not yet created. Please create your Email Wallet first on https://emailwallet.org.".to_string();
        let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "errorMsg": error_msg.clone(), "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
        let body_html = render_html("error.html", render_data).await?;
        let email = EmailMessage {
            subject,
            to: request.email_addr,
            body_plain: error_msg,
            body_html,
            reference: None,
            reply_to: None,
            body_attachments: None,
        };
        return Ok((request_id, email));
    }
    let account_code = AccountCode(hex2field(&account_code_str.unwrap())?);
    let account_salt = AccountSalt::new(
        &PaddedEmailAddr::from_email_addr(&request.email_addr),
        account_code,
    )?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    let body_plain = format!(
        "Hi {}! Please reply to this email to sign-in {}.\nYou don't have to add any message in the reply 😄.\nYour wallet address: {}/address/{}.",
        request.email_addr, request.username, CHAIN_RPC_EXPLORER.get().unwrap(), wallet_addr,
    );
    let render_data = serde_json::json!({"userEmailAddr": request.email_addr, "originalSubject": subject, "walletAddr": wallet_addr, "chainRPCExplorer": CHAIN_RPC_EXPLORER.get().unwrap()});
    let body_html = render_html("send_request.html", render_data).await?;
    let email = EmailMessage {
        subject: subject.clone(),
        body_html,
        body_plain,
        to: request.email_addr,
        reference: None,
        reply_to: None,
        body_attachments: None,
    };
    Ok((request_id, email))
}

pub async fn register_ephe_addr(payload: String) -> Result<U256> {
    let request = serde_json::from_str::<RegisterEpheAddrRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let wallet_addr = Address::from_str(&request.wallet_addr)?;
    let ephe_addr = Address::from_str(&request.ephe_addr)?;
    let signature = Bytes::from_str(&request.signature)?;
    let (tx_hash, nonce) = CLIENT
        .register_ephe_addr_for_wallet(wallet_addr, request.username.clone(), ephe_addr, signature)
        .await?;
    trace!(
        LOG,
        "Register ephe addr tx hash: {}, nonce: {}, request: {:?}",
        tx_hash,
        nonce,
        request
    );
    Ok(nonce)
}

pub async fn execute_ephemeral_tx(payload: String) -> Result<String> {
    let request = serde_json::from_str::<ExecuteEphemeralTxRequest>(&payload)
        .map_err(|_| anyhow!("Invalid payload json".to_string()))?;
    let tx = EphemeralTx {
        wallet_addr: Address::from_str(&request.wallet_addr)?,
        tx_nonce: U256::from_str_radix(&request.tx_nonce, 10)?,
        ephe_addr: Address::from_str(&request.ephe_addr)?,
        ephe_addr_nonce: U256::from_str_radix(&request.ephe_addr_nonce, 10)?,
        target: Address::from_str(&request.target)?,
        eth_value: U256::from_str_radix(&request.eth_value, 10)?,
        data: Bytes::from_str(&request.data)?,
        token_amount: U256::from_str_radix(&request.token_amount, 10)?,
        signature: Bytes::from_str(&request.signature)?,
    };
    let tx_hash = CLIENT.execute_ephemeral_tx(tx).await?;
    trace!(
        LOG,
        "Execute ephemeral tx hash: {}, request: {:?}",
        tx_hash,
        request
    );
    Ok(tx_hash)
}
