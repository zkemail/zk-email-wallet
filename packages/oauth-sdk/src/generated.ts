//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// EmailWalletCore
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const emailWalletCoreAbi = [
  { type: 'constructor', inputs: [], stateMutability: 'nonpayable' },
  { type: 'fallback', stateMutability: 'payable' },
  { type: 'receive', stateMutability: 'payable' },
  {
    type: 'function',
    inputs: [],
    name: 'accountHandler',
    outputs: [{ name: '', internalType: 'contract AccountHandler', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'depositTokenAsExtension',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'emailNullifiers',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'emailValidityDuration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'target', internalType: 'address', type: 'address' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'executeAsExtension',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'extensionHandler',
    outputs: [{ name: '', internalType: 'contract ExtensionHandler', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'emailOp',
        internalType: 'struct EmailOp',
        type: 'tuple',
        components: [
          { name: 'accountSalt', internalType: 'bytes32', type: 'bytes32' },
          { name: 'command', internalType: 'string', type: 'string' },
          { name: 'emailNullifier', internalType: 'bytes32', type: 'bytes32' },
          { name: 'emailDomain', internalType: 'string', type: 'string' },
          { name: 'dkimPublicKeyHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'maskedSubject', internalType: 'string', type: 'string' },
          { name: 'skipSubjectPrefix', internalType: 'uint256', type: 'uint256' },
          { name: 'timestamp', internalType: 'uint256', type: 'uint256' },
          { name: 'hasEmailRecipient', internalType: 'bool', type: 'bool' },
          { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'numRecipientEmailAddrBytes', internalType: 'uint256', type: 'uint256' },
          { name: 'recipientETHAddr', internalType: 'address', type: 'address' },
          { name: 'feeTokenName', internalType: 'string', type: 'string' },
          { name: 'feePerGas', internalType: 'uint256', type: 'uint256' },
          { name: 'executeCallData', internalType: 'bytes', type: 'bytes' },
          { name: 'extensionName', internalType: 'string', type: 'string' },
          { name: 'newWalletOwner', internalType: 'address', type: 'address' },
          { name: 'newDkimRegistry', internalType: 'address', type: 'address' },
          {
            name: 'walletParams',
            internalType: 'struct WalletParams',
            type: 'tuple',
            components: [
              { name: 'tokenName', internalType: 'string', type: 'string' },
              { name: 'amount', internalType: 'uint256', type: 'uint256' },
            ],
          },
          {
            name: 'extensionParams',
            internalType: 'struct ExtensionParams',
            type: 'tuple',
            components: [
              { name: 'subjectTemplateIndex', internalType: 'uint8', type: 'uint8' },
              { name: 'subjectParams', internalType: 'bytes[]', type: 'bytes[]' },
            ],
          },
          { name: 'emailProof', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'handleEmailOp',
    outputs: [
      { name: 'success', internalType: 'bool', type: 'bool' },
      { name: 'err', internalType: 'bytes', type: 'bytes' },
      { name: 'totalFeeInETH', internalType: 'uint256', type: 'uint256' },
      { name: 'registeredUnclaimId', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_relayerHandler', internalType: 'address', type: 'address' },
      { name: '_accountHandler', internalType: 'address', type: 'address' },
      { name: '_unclaimsHandler', internalType: 'address', type: 'address' },
      { name: '_extensionHandler', internalType: 'address', type: 'address' },
      { name: '_verifier', internalType: 'address', type: 'address' },
      { name: '_tokenRegistry', internalType: 'address', type: 'address' },
      { name: '_priceOracle', internalType: 'address', type: 'address' },
      { name: '_wethContract', internalType: 'address', type: 'address' },
      { name: '_maxFeePerGas', internalType: 'uint256', type: 'uint256' },
      { name: '_emailValidityDuration', internalType: 'uint256', type: 'uint256' },
      { name: '_unclaimedFundClaimGas', internalType: 'uint256', type: 'uint256' },
      { name: '_unclaimedStateClaimGas', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'initialize',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'defaultExtensions', internalType: 'bytes[]', type: 'bytes[]' }],
    name: 'initializeExtension',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maxFeePerGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'priceOracle',
    outputs: [{ name: '', internalType: 'contract IPriceOracle', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'proxiableUUID',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'registerUnclaimedStateAsExtension',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'relayerHandler',
    outputs: [{ name: '', internalType: 'contract RelayerHandler', type: 'address' }],
    stateMutability: 'view',
  },
  { type: 'function', inputs: [], name: 'renounceOwnership', outputs: [], stateMutability: 'nonpayable' },
  {
    type: 'function',
    inputs: [
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'requestTokenAsExtension',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'tokenRegistry',
    outputs: [{ name: '', internalType: 'contract TokenRegistry', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'unclaimedFundClaimGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'unclaimedStateClaimGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'unclaimsHandler',
    outputs: [{ name: '', internalType: 'contract UnclaimsHandler', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'newImplementation', internalType: 'address', type: 'address' }],
    name: 'upgradeTo',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newImplementation', internalType: 'address', type: 'address' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'upgradeToAndCall',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'emailOp',
        internalType: 'struct EmailOp',
        type: 'tuple',
        components: [
          { name: 'accountSalt', internalType: 'bytes32', type: 'bytes32' },
          { name: 'command', internalType: 'string', type: 'string' },
          { name: 'emailNullifier', internalType: 'bytes32', type: 'bytes32' },
          { name: 'emailDomain', internalType: 'string', type: 'string' },
          { name: 'dkimPublicKeyHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'maskedSubject', internalType: 'string', type: 'string' },
          { name: 'skipSubjectPrefix', internalType: 'uint256', type: 'uint256' },
          { name: 'timestamp', internalType: 'uint256', type: 'uint256' },
          { name: 'hasEmailRecipient', internalType: 'bool', type: 'bool' },
          { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'numRecipientEmailAddrBytes', internalType: 'uint256', type: 'uint256' },
          { name: 'recipientETHAddr', internalType: 'address', type: 'address' },
          { name: 'feeTokenName', internalType: 'string', type: 'string' },
          { name: 'feePerGas', internalType: 'uint256', type: 'uint256' },
          { name: 'executeCallData', internalType: 'bytes', type: 'bytes' },
          { name: 'extensionName', internalType: 'string', type: 'string' },
          { name: 'newWalletOwner', internalType: 'address', type: 'address' },
          { name: 'newDkimRegistry', internalType: 'address', type: 'address' },
          {
            name: 'walletParams',
            internalType: 'struct WalletParams',
            type: 'tuple',
            components: [
              { name: 'tokenName', internalType: 'string', type: 'string' },
              { name: 'amount', internalType: 'uint256', type: 'uint256' },
            ],
          },
          {
            name: 'extensionParams',
            internalType: 'struct ExtensionParams',
            type: 'tuple',
            components: [
              { name: 'subjectTemplateIndex', internalType: 'uint8', type: 'uint8' },
              { name: 'subjectParams', internalType: 'bytes[]', type: 'bytes[]' },
            ],
          },
          { name: 'emailProof', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'validateEmailOp',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'verifier',
    outputs: [{ name: '', internalType: 'contract IVerifier', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'wethContract',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousAdmin', internalType: 'address', type: 'address', indexed: false },
      { name: 'newAdmin', internalType: 'address', type: 'address', indexed: false },
    ],
    name: 'AdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [{ name: 'beacon', internalType: 'address', type: 'address', indexed: true }],
    name: 'BeaconUpgraded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'success', internalType: 'bool', type: 'bool', indexed: true },
      { name: 'registeredUnclaimId', internalType: 'uint256', type: 'uint256', indexed: true },
      { name: 'emailNullifier', internalType: 'bytes32', type: 'bytes32', indexed: true },
      { name: 'accountSalt', internalType: 'bytes32', type: 'bytes32', indexed: false },
      { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32', indexed: false },
      { name: 'recipientETHAddr', internalType: 'address', type: 'address', indexed: false },
      { name: 'err', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'EmailOpHandled',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [{ name: 'version', internalType: 'uint8', type: 'uint8', indexed: false }],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [{ name: 'implementation', internalType: 'address', type: 'address', indexed: true }],
    name: 'Upgraded',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IOauth
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iOauthAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getInfoOfWalletAndNonce',
    outputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
    ],
    name: 'getTokenAllowancesOfWalletAndNonce',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'wallet', internalType: 'address', type: 'address' }],
    name: 'getUsernameOfWallet',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'username', internalType: 'string', type: 'string' }],
    name: 'getWalletOfUsername',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'reduceTokenAllowance',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address' },
      { name: 'epheAddr', internalType: 'address', type: 'address' },
    ],
    name: 'registerEpheAddr',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'username', internalType: 'string', type: 'string' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
      {
        name: 'tokenAllowances',
        internalType: 'struct TokenAllowance[]',
        type: 'tuple[]',
        components: [
          { name: 'tokenAddr', internalType: 'address', type: 'address' },
          { name: 'amount', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'signin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'username', internalType: 'string', type: 'string' }],
    name: 'signup',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address' },
      { name: 'epheAddr', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'validateEpheAddr',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'epheAddr', internalType: 'address', type: 'address' },
      { name: 'hash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'signature', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'validateSignature',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address', indexed: true },
      { name: 'token', internalType: 'address', type: 'address', indexed: true },
      { name: 'amount', internalType: 'uint256', type: 'uint256', indexed: true },
      { name: 'nonce', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'ReducedTokenAllowance',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address', indexed: true },
      { name: 'epheAddr', internalType: 'address', type: 'address', indexed: true },
      { name: 'nonce', internalType: 'uint256', type: 'uint256', indexed: true },
      { name: 'username', internalType: 'string', type: 'string', indexed: false },
    ],
    name: 'RegisteredEpheAddr',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address', indexed: true },
      { name: 'username', internalType: 'string', type: 'string', indexed: true },
      { name: 'nonce', internalType: 'uint256', type: 'uint256', indexed: true },
      { name: 'expiry', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'tokenAllowances',
        internalType: 'struct TokenAllowance[]',
        type: 'tuple[]',
        components: [
          { name: 'tokenAddr', internalType: 'address', type: 'address' },
          { name: 'amount', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'Signin',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'wallet', internalType: 'address', type: 'address', indexed: true },
      { name: 'username', internalType: 'string', type: 'string', indexed: true },
    ],
    name: 'Signup',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Wallet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const walletAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: 'wethAddress', internalType: 'address', type: 'address' },
      { name: 'oauthAddress', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'nonpayable',
  },
  { type: 'fallback', stateMutability: 'payable' },
  { type: 'receive', stateMutability: 'payable' },
  {
    type: 'function',
    inputs: [],
    name: 'epheTxNonce',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'target', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'execute',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'txData',
        internalType: 'struct EphemeralTx',
        type: 'tuple',
        components: [
          { name: 'walletAddr', internalType: 'address', type: 'address' },
          { name: 'txNonce', internalType: 'uint256', type: 'uint256' },
          { name: 'epheAddr', internalType: 'address', type: 'address' },
          { name: 'epheAddrNonce', internalType: 'uint256', type: 'uint256' },
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'ethValue', internalType: 'uint256', type: 'uint256' },
          { name: 'data', internalType: 'bytes', type: 'bytes' },
          { name: 'tokenAmount', internalType: 'uint256', type: 'uint256' },
          { name: 'signature', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'executeEphemeralTx',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getOauth',
    outputs: [{ name: '', internalType: 'contract IOauth', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'txData',
        internalType: 'struct EphemeralTx',
        type: 'tuple',
        components: [
          { name: 'walletAddr', internalType: 'address', type: 'address' },
          { name: 'txNonce', internalType: 'uint256', type: 'uint256' },
          { name: 'epheAddr', internalType: 'address', type: 'address' },
          { name: 'epheAddrNonce', internalType: 'uint256', type: 'uint256' },
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'ethValue', internalType: 'uint256', type: 'uint256' },
          { name: 'data', internalType: 'bytes', type: 'bytes' },
          { name: 'tokenAmount', internalType: 'uint256', type: 'uint256' },
          { name: 'signature', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'hashEphemeralTx',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'pure',
  },
  { type: 'function', inputs: [], name: 'initialize', outputs: [], stateMutability: 'nonpayable' },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256[]', type: 'uint256[]' },
      { name: '', internalType: 'uint256[]', type: 'uint256[]' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC1155BatchReceived',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC1155Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC721Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'proxiableUUID',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  { type: 'function', inputs: [], name: 'renounceOwnership', outputs: [], stateMutability: 'nonpayable' },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'tokensReceived',
    outputs: [],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newImplementation', internalType: 'address', type: 'address' }],
    name: 'upgradeTo',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newImplementation', internalType: 'address', type: 'address' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'upgradeToAndCall',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousAdmin', internalType: 'address', type: 'address', indexed: false },
      { name: 'newAdmin', internalType: 'address', type: 'address', indexed: false },
    ],
    name: 'AdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [{ name: 'beacon', internalType: 'address', type: 'address', indexed: true }],
    name: 'BeaconUpgraded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [{ name: 'version', internalType: 'uint8', type: 'uint8', indexed: false }],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [{ name: 'implementation', internalType: 'address', type: 'address', indexed: true }],
    name: 'Upgraded',
  },
] as const
