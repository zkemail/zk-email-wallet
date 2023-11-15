pub use account_handler::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod account_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_relayerHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_defaultDkimRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_walletImplementation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_emailValidityDuration"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accountKeyCommitOfPointer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "accountKeyCommitOfPointer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createAccount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailAddrPointer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountKeyCommit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("psiPoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Wallet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultDkimRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultDkimRegistry",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDKIMRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dkimRegistryOfWalletSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dkimRegistryOfWalletSalt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emailNullifiers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emailNullifiers"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emailValidityDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "emailValidityDuration",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInfoOfAccountKeyCommit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getInfoOfAccountKeyCommit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountKeyCommit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct AccountKeyInfo"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWalletOfEmailAddrPointer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getWalletOfEmailAddrPointer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailAddrPointer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWalletOfSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getWalletOfSalt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("infoOfAccountKeyCommit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "infoOfAccountKeyCommit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("relayer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialized"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initializeAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initializeAccount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailAddrPointer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailNullifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dkimPublicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDKIMPublicKeyHashValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDKIMPublicKeyHashValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emailDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pointerOfPSIPoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pointerOfPSIPoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("relayerHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relayerHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract RelayerHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transportAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transportAccount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldAccountKeyCommit",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newEmailAddrPointer",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newAccountKeyCommit",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPSIPoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "transportEmailProof",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct EmailProof"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "accountCreationProof",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDKIMRegistryOfWalletSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateDKIMRegistryOfWalletSalt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dkimRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVerifier"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("walletImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "walletImplementation",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ACCOUNTHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0,\xFA8\x03\x80b\0,\xFA\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\0\xD4V[b\0\0@3b\0\0gV[a\x01\0R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x82\x16`\xA0R\x91\x81\x16`\xC0R\x16`\xE0Rb\0\x01;V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xCFW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\0\xEDW`\0\x80\xFD[b\0\0\xF8\x86b\0\0\xB7V[\x94Pb\0\x01\x08` \x87\x01b\0\0\xB7V[\x93Pb\0\x01\x18`@\x87\x01b\0\0\xB7V[\x92Pb\0\x01(``\x87\x01b\0\0\xB7V[\x91P`\x80\x86\x01Q\x90P\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa+\x03b\0\x01\xF7`\09`\0\x81\x81a\x03\x94\x01R\x81\x81a\x0C\x11\x01Ra\x14\x84\x01R`\0\x81\x81a\x01\x8F\x01R\x81\x81a\x08\x17\x01R\x81\x81a\r\x99\x01R\x81\x81a\x0F2\x01Ra\x15s\x01R`\0\x81\x81a\x01\xE7\x01R\x81\x81a\x06\x06\x01R\x81\x81a\x08F\x01R\x81\x81a\nH\x01R\x81\x81a\r\xC8\x01R\x81\x81a\x0Fc\x01R\x81\x81a\x0F\xF7\x01R\x81\x81a\x12\xA8\x01Ra\x15\xA6\x01R`\0\x81\x81a\x02\xE2\x01R\x81\x81a\x05V\x01Ra\x19o\x01R`\0\x81\x81a\x04E\x01Ra\x17\xA9\x01Ra+\x03`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x01\x1EW`\x005`\xE0\x1C\x80c\x01&-\xD2\x14b\0\x01#W\x80c\x17\xAC\xE6\xB5\x14b\0\x01RW\x80c+z\xC3\xF3\x14b\0\x01\x89W\x80c@\xD2\0A\x14b\0\x01\xB1W\x80cVd\xC7\x8E\x14b\0\x01\xCAW\x80c^_&\x10\x14b\0\x01\xE1W\x80ck\x0F\x04}\x14b\0\x02\tW\x80cm\xD5\x0F\x8B\x14b\0\x02\xA4W\x80cqP\x18\xA6\x14b\0\x02\xBBW\x80cr\x1CI\x96\x14b\0\x02\xC5W\x80c\x81\x17\xAB\xC1\x14b\0\x02\xDCW\x80c\x8D\xA5\xCB[\x14b\0\x03\x04W\x80c\xA9 \x14\xDD\x14b\0\x03\x0EW\x80c\xA9\x9Bj\xC6\x14b\0\x03KW\x80c\xAC\xAE\x05\xFE\x14b\0\x03wW\x80c\xB9\r6\xF9\x14b\0\x03\x8EW\x80c\xBAY\x1A6\x14b\0\x03\xB6W\x80c\xBFh\xC3\x06\x14b\0\x03\xD9W\x80c\xD3C\xD5\xCA\x14b\0\x04?W\x80c\xD8BHN\x14b\0\x04gW\x80c\xF2\xFD\xE3\x8B\x14b\0\x04~W[`\0\x80\xFD[b\0\x01:b\0\x0146`\x04b\0\x1A\xC4V[b\0\x04\x95V[`@Qb\0\x01I\x91\x90b\0\x1A\xDEV[`@Q\x80\x91\x03\x90\xF3[b\0\x01xb\0\x01c6`\x04b\0\x1A\xC4V[`\x05` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01b\0\x01IV[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x01\xC8b\0\x01\xC26`\x04b\0\x1B\x0FV[b\0\x04\xC3V[\0[b\0\x01:b\0\x01\xDB6`\x04b\0\x1A\xC4V[b\0\x04\xFBV[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x02vb\0\x02\x1A6`\x04b\0\x1A\xC4V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R\x93\x84R`\x03\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x85R`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15\x91\x84\x01\x91\x90\x91R`\x01\x01T\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x84\x01Q\x15\x15\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01b\0\x01IV[b\0\x01:b\0\x02\xB56`\x04b\0\x1B\x89V[b\0\x05\xBBV[b\0\x01\xC8b\0\n\x16V[b\0\x01\xC8b\0\x02\xD66`\x04b\0\x1C\xF1V[b\0\n.V[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x01:b\0\x12JV[b\0\x03<b\0\x03\x1F6`\x04b\0\x1E\x15V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[`@Q\x90\x81R` \x01b\0\x01IV[b\0\x01:b\0\x03\\6`\x04b\0\x1A\xC4V[`\x04` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[b\0\x01\xC8b\0\x03\x886`\x04b\0\x1EUV[b\0\x12YV[b\0\x03<\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x03<b\0\x03\xC76`\x04b\0\x1A\xC4V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[b\0\x04\x19b\0\x03\xEA6`\x04b\0\x1A\xC4V[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x82\x16\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90\x83V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x15\x15` \x84\x01R\x90\x82\x01R``\x01b\0\x01IV[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x01xb\0\x04x6`\x04b\0\x1E\xF1V[b\0\x17\x88V[b\0\x01\xC8b\0\x04\x8F6`\x04b\0\x1FDV[b\0\x18FV[`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 T\x84R`\x03\x90\x91R\x82 \x01Tb\0\x04\xBD\x90b\0\x04\xFBV[\x92\x91PPV[b\0\x04\xCDb\0\x18\xC5V[`\0\x91\x82R`\x04` R`@\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0b\0\x04\xBD\x82`@Q\x80` \x01b\0\x05\x14\x90b\0\x1A\xB6V[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x04`\x1F\x90\x92\x01\x16\x90\x81R`$\x81\x01`@\x90\x81R` \x82\x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c J\x7F\x07`\xE2\x1B\x17\x90R\x90Qb\0\x05\x7F\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x90\x91\x01b\0\x1F\xB6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x05\x9F\x92\x91` \x01b\0\x1F\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 b\0\x19(V[`\0cei!\xFFB\x10b\0\x05\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 \x0FV[`@Q\x80\x91\x03\x90\xFD[`@Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\x06=\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\x81\x91\x90b\0 [V[\x03b\0\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 uV[`\0\x88\x81R`\x01` R`@\x90 T\x15b\0\x06\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmpointer exists`\x90\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x80\x1B`\x02\x86\x86`@Qb\0\x07\x08\x92\x91\x90b\0 \xA5V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14b\0\x07YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoPSI point exists`\x80\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x87\x81R`\x03` R`@\x90 `\x01\x01T\x15b\0\x07\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpwalletSalt exists`x\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[b\0\x07\xCCb\0\x07\xBD\x87b\0\x04\xFBV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x15b\0\x08\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x1D\xD8[\x1B\x19]\x08\x18[\x1C\x99XY\x1EH\x19\x19\\\x1B\x1B\xDEYY`J\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x99\x9A\xB5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c?V\x15\xA43`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x08\x92\x91\x90b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x08\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x08\xD6\x91\x90b\0 [V[\x8A\x8A\x8A\x8A\x8A\x8A\x8A`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\t\x02\x98\x97\x96\x95\x94\x93\x92\x91\x90b\0 \xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\t W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\tF\x91\x90b\0!0V[b\0\teW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!TV[`\0\x88\x81R`\x01` \x81\x81R`@\x80\x84 \x8B\x90U\x8A\x84R`\x03\x90\x91R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x81U\x01\x87\x90UQ\x88\x90`\x02\x90b\0\t\xAE\x90\x88\x90\x88\x90b\0 \xA5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ub\0\t\xC9\x86b\0\x19>V[\x90P\x85\x7F\xA6~\xDF\xB1WIs\xCC\x13\xEB\xF7\xC1x2\x8E\xC2\t|L\x16M\x95\x95\xC0\x06\xE6\\\x0F\xF0/\xBAf\x89\x89\x88\x88`@Qb\0\n\x03\x94\x93\x92\x91\x90b\0!\x8BV[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[b\0\n b\0\x18\xC5V[b\0\n,`\0b\0\x1A<V[V[`@Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\n\x7F\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\n\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\n\xC3\x91\x90b\0 [V[\x03b\0\n\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 uV[`\0\x86\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16b\0\x0BFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x1B\xDB\x19\x08\x1C\x99[\x18^Y\\\x88\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`2\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x86\x81R`\x03` R`@\x90 T3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03b\0\x0B\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Rynew relayer cannot be same`0\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x86\x81R`\x03` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16b\0\x0C\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x18X\xD8\xDB\xDD[\x9D\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`J\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83` \x01Qb\0\x0CA\x91\x90b\0!\xB7V[\x11b\0\x0CaW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!\xD9V[`@\x80\x83\x01Q`\0\x90\x81R`\x05` R T`\xFF\x16\x15b\0\x0C\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0\"\0V[`\0\x85\x81R`\x01` R`@\x90 T\x15b\0\r$W`\0\x84\x81R`\x03` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15b\0\r\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fnew account is already initializ`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01b\0\x05\xE3V[b\0\r\x97V[`\0\x80\x1B`\x02\x84`@Qb\0\r:\x91\x90b\0\")V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14b\0\r\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{new PSI point already exists` \x1B`D\x82\x01R`d\x01b\0\x05\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x99\x9A\xB5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c?V\x15\xA43`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x0E\x14\x91\x90b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0EX\x91\x90b\0 [V[`\0\x89\x81R`\x03` R`@\x90\x81\x90 `\x01\x01T\x90Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Rb\0\x0E\x97\x92\x91\x8A\x91\x8A\x91\x90\x8A\x90\x89\x90`\x04\x01b\0\"GV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\xDB\x91\x90b\0!0V[b\0\x0E\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!TV[\x81Q``\x83\x01Q` \x80\x85\x01Q`@\x80\x87\x01Q`\0\x8C\x81R`\x03\x90\x94R\x92\x81\x90 T\x90Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x96c\x87\xFA\xC4\xBA\x96\x90\x95\x90\x94\x93\x90\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x92c?V\x15\xA4\x92b\0\x0F\x9C\x92\x91\x16\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0F\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0F\xE0\x91\x90b\0 [V[`@Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\x10.\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x10LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10r\x91\x90b\0 [V[\x8D\x8C\x8B`\x80\x01Q`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x10\x9F\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90b\0\"\x95V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x10\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10\xE3\x91\x90b\0!0V[b\0\x111W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid account transport proof\0`D\x82\x01R`d\x01b\0\x05\xE3V[`@\x80\x83\x01Q`\0\x90\x81R`\x05` \x90\x81R\x82\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x88\x83R\x90R T\x15b\0\x11\x8DW`\0\x85\x81R`\x01` \x81\x81R`@\x80\x84 T\x84R`\x03\x90\x91R\x82 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U\x01U[`\0\x85\x81R`\x01` R`@\x90\x81\x90 \x85\x90UQ\x85\x90`\x02\x90b\0\x11\xB3\x90\x86\x90b\0\")V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 \x93\x90\x93U`\0\x89\x81R`\x03\x90\x92R\x80\x82 `\x01\x90\x81\x01T\x88\x84R\x91\x90\x92 \x91\x82\x01U\x80T`\x01`\xA0\x1B`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x163`\xFF`\xA0\x1B\x19\x16\x17\x17\x90U\x7F\xB0\xB7\xC3J<\xD8:\xFC<\xD4\xBC\xB5\xFD\xAFn\x9Fl\x94\xA5\xA5\xCCgV\x89\xED\x14G\xB2\xAE\xA9\xAC\x0E\x90b\0\x12:\x90\x88\x90\x88\x90\x88\x90\x88\x90b\0\"\xFCV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[cei!\xFFB\x10b\0\x12\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 \x0FV[`\0\x88\x81R`\x01` R`@\x80\x82 T\x90Qc\x0F\xD5\x85i`\xE2\x1B\x81R\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\x12\xDF\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x12\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x13#\x91\x90b\0 [V[\x03b\0\x13CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 uV[\x80b\0\x13\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x18X\xD8\xDB\xDD[\x9D\x08\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`R\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x13\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn4\xB7;0\xB64\xB2\x1092\xB60\xBC\xB2\xB9`\x89\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15b\0\x14KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x18X\xD8\xDB\xDD[\x9D\x08\x18[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`*\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x85\x81R`\x05` R`@\x90 T`\xFF\x16\x15b\0\x14}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0\"\0V[Bb\0\x14\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88b\0!\xB7V[\x11b\0\x14\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!\xD9V[b\0\x15$`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pb\0\x17\x88\x91PPV[b\0\x15qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{\r-\xCE\xCC-\x8D,\x84\x08\x89i)\xA4\x0E\x0E\xACM\x8D,d\rl\xAF$\r\x0C.m`#\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCE.\x9F\x0E\x89\x89\x87\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c?V\x15\xA43`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x15\xF2\x91\x90b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x16\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x166\x91\x90b\0 [V[\x8F\x88\x8D\x8C\x8C`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x16b\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90b\0##V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x16\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x16\xA6\x91\x90b\0!0V[b\0\x17\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Finvalid account initialization p`D\x82\x01Rc97\xB7\xB3`\xE1\x1B`d\x82\x01R`\x84\x01b\0\x05\xE3V[`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U\x89\x85R`\x05\x83R\x81\x85 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x94\x86\x90R\x92\x82R\x92\x90\x91\x01T\x82Q\x8C\x81R\x91\x82\x01\x84\x90R\x91\x7F\x1A\xBA\x97\x02NW\x05T\x19\xDDN\x116\x1F\x9Ev\xF2\xBC\xEC\xF0\xE4\xD9\xA5\x1D\xE1\xC0\xE9\xDFfA\xD8\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPV[`\0\x83\x81R`\x04` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x17\xC9WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Qcs\xD3\xCB\xBD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xE7\xA7\x97z\x90b\0\x17\xF9\x90\x87\x90\x87\x90`\x04\x01b\0#\x86V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x18\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x18=\x91\x90b\0!0V[\x95\x94PPPPPV[b\0\x18Pb\0\x18\xC5V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x18\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x05\xE3V[b\0\x18\xC2\x81b\0\x1A<V[PV[3b\0\x18\xD0b\0\x12JV[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\n,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x05\xE3V[`\0b\0\x197\x83\x830b\0\x1A\x8CV[\x93\x92PPPV[`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c J\x7F\x07`\xE2\x1B\x17\x90R\x90Q`\0\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90b\0\x19\x9B\x90b\0\x1A\xB6V[b\0\x19\xA8\x92\x91\x90b\0\x1F\xB6V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x19\xC9W=`\0\x80>=`\0\xFD[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xFD\xE3\x8Bb\0\x19\xE5b\0\x12JV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x1A\x03\x91\x90b\0\x1A\xDEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1A\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1A3W=`\0\x80>=`\0\xFD[PPPP\x91\x90PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0`@Q\x83`@\x82\x01R\x84` \x82\x01R\x82\x81R`\x0B\x81\x01\x90P`\xFF\x81S`U\x90 \x94\x93PPPPV[a\x07#\x80b\0#\xAB\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x1A\xD7W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x1B\nW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x1B#W`\0\x80\xFD[\x825\x91Pb\0\x1B5` \x84\x01b\0\x1A\xF2V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0\x1BQW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x1BiW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15b\0\x1B\x82W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15b\0\x1B\xA5W`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1B\xD2W`\0\x80\xFD[b\0\x1B\xE0\x8B\x83\x8C\x01b\0\x1B>V[\x90\x96P\x94P`\x80\x8A\x015\x91P\x80\x82\x11\x15b\0\x1B\xFAW`\0\x80\xFD[Pb\0\x1C\t\x8A\x82\x8B\x01b\0\x1B>V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x1CWWb\0\x1CWb\0\x1C\x1CV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x1CoW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1C\x8CWb\0\x1C\x8Cb\0\x1C\x1CV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x1C\xB7Wb\0\x1C\xB7b\0\x1C\x1CV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x1C\xD1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x1D\x0BW`\0\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1D8W`\0\x80\xFD[b\0\x1DF\x8A\x83\x8B\x01b\0\x1C]V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15b\0\x1D]W`\0\x80\xFD[\x90\x88\x01\x90`\xA0\x82\x8B\x03\x12\x15b\0\x1DrW`\0\x80\xFD[b\0\x1D|b\0\x1C2V[\x825\x82\x81\x11\x15b\0\x1D\x8CW`\0\x80\xFD[b\0\x1D\x9A\x8C\x82\x86\x01b\0\x1C]V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15b\0\x1D\xCEW`\0\x80\xFD[b\0\x1D\xDC\x8C\x82\x86\x01b\0\x1C]V[`\x80\x83\x01RP\x93P`\xA0\x89\x015\x91P\x80\x82\x11\x15b\0\x1D\xF9W`\0\x80\xFD[Pb\0\x1E\x08\x89\x82\x8A\x01b\0\x1C]V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15b\0\x1E(W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x1E?W`\0\x80\xFD[b\0\x1EM\x84\x82\x85\x01b\0\x1C]V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15b\0\x1ErW`\0\x80\xFD[\x885\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1E\x91W`\0\x80\xFD[b\0\x1E\x9F\x8C\x83\x8D\x01b\0\x1B>V[\x90\x99P\x97P`@\x8B\x015\x96P``\x8B\x015\x95P`\x80\x8B\x015\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15b\0\x1E\xCEW`\0\x80\xFD[Pb\0\x1E\xDD\x8B\x82\x8C\x01b\0\x1B>V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x1F\x07W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x1F%W`\0\x80\xFD[b\0\x1F3\x86\x82\x87\x01b\0\x1C]V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x1FWW`\0\x80\xFD[b\0\x197\x82b\0\x1A\xF2V[`\0[\x83\x81\x10\x15b\0\x1F\x7FW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x1FeV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Rb\0\x1F\xA2\x81` \x86\x01` \x86\x01b\0\x1FbV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90b\0\x1EM\x90\x83\x01\x84b\0\x1F\x88V[`\0\x83Qb\0\x1F\xF0\x81\x84` \x88\x01b\0\x1FbV[\x83Q\x90\x83\x01\x90b\0 \x06\x81\x83` \x88\x01b\0\x1FbV[\x01\x94\x93PPPPV[` \x80\x82R`,\x90\x82\x01R\x7Fthis function is not allowed fro`@\x82\x01Rkm 2023-12-01`\xA0\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0 nW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`\x16\x90\x82\x01Ru\x1C\x99[\x18^Y\\\x88\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`R\x1B`@\x82\x01R``\x01\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`\xC0`\x80\x82\x01R`\0b\0!\x0C`\xC0\x83\x01\x86\x88b\0 \xB5V[\x82\x81\x03`\xA0\x84\x01Rb\0!!\x81\x85\x87b\0 \xB5V[\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0!CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x197W`\0\x80\xFD[` \x80\x82R`\x1E\x90\x82\x01R\x7Finvalid account creation proof\0\0`@\x82\x01R``\x01\x90V[\x84\x81R\x83` \x82\x01R```@\x82\x01R`\0b\0!\xAD``\x83\x01\x84\x86b\0 \xB5V[\x96\x95PPPPPPV[\x80\x82\x01\x80\x82\x11\x15b\0\x04\xBDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[` \x80\x82R`\r\x90\x82\x01Rl\x19[XZ[\x08\x19^\x1C\x1A\\\x99Y`\x9A\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x0F\x90\x82\x01Rn\x19[XZ[\x08\x1B\x9D[\x1B\x1AY\x9AYY`\x8A\x1B`@\x82\x01R``\x01\x90V[`\0\x82Qb\0\"=\x81\x84` \x87\x01b\0\x1FbV[\x91\x90\x91\x01\x92\x91PPV[\x86\x81R\x85` \x82\x01R\x84`@\x82\x01R\x83``\x82\x01R`\xC0`\x80\x82\x01R`\0b\0\"t`\xC0\x83\x01\x85b\0\x1F\x88V[\x82\x81\x03`\xA0\x84\x01Rb\0\"\x88\x81\x85b\0\x1F\x88V[\x99\x98PPPPPPPPPV[`\0a\x01 \x80\x83Rb\0\"\xAB\x81\x84\x01\x8Db\0\x1F\x88V[\x90P\x8A` \x84\x01R\x89`@\x84\x01R\x88``\x84\x01R\x87`\x80\x84\x01R\x86`\xA0\x84\x01R\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Rb\0\"\xEC\x81\x85b\0\x1F\x88V[\x9C\x9BPPPPPPPPPPPPV[\x84\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0b\0!\xAD`\x80\x83\x01\x84b\0\x1F\x88V[`\0a\x01\0\x80\x83Rb\0#:\x81\x84\x01\x8D\x8Fb\0 \xB5V[\x90P\x8A` \x84\x01R\x89`@\x84\x01R\x88``\x84\x01R\x87`\x80\x84\x01R\x86`\xA0\x84\x01R\x85`\xC0\x84\x01R\x82\x81\x03`\xE0\x84\x01Rb\0#u\x81\x85\x87b\0 \xB5V[\x9D\x9CPPPPPPPPPPPPPV[`@\x81R`\0b\0#\x9B`@\x83\x01\x85b\0\x1F\x88V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE`\x80`@R`@Qa\x07#8\x03\x80a\x07#\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x03\x17V[a\0.\x82\x82`\0a\x005V[PPa\x044V[a\0>\x83a\0kV[`\0\x82Q\x11\x80a\0KWP\x80[\x15a\0fWa\0d\x83\x83a\0\xAB` \x1Ba\0)\x17` \x1CV[P[PPPV[a\0t\x81a\0\xD7V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\0\xD0\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x06\xFC`'\x919a\x01\xA9V[\x93\x92PPPV[a\0\xEA\x81a\x02\"` \x1Ba\0U\x17` \x1CV[a\x01QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x80a\x01\x88\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x021` \x1Ba\0d\x17` \x1CV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x01\xC6\x91\x90a\x03\xE5V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x02\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x06V[``\x91P[P\x90\x92P\x90Pa\x02\x18\x86\x83\x83\x87a\x024V[\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[``\x83\x15a\x02\xA1W\x82Q`\0\x03a\x02\x9AWa\x02N\x85a\x02\"V[a\x02\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01HV[P\x81a\x02\xABV[a\x02\xAB\x83\x83a\x02\xB3V[\x94\x93PPPPV[\x81Q\x15a\x02\xC3W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01H\x91\x90a\x04\x01V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x03\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xF6V[PP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x03*W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03AW`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x03^W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x03rW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x03\x84Wa\x03\x84a\x02\xDDV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x03\xACWa\x03\xACa\x02\xDDV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x03\xC5W`\0\x80\xFD[a\x03\xD6\x83` \x83\x01` \x88\x01a\x02\xF3V[\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x82Qa\x03\xF7\x81\x84` \x87\x01a\x02\xF3V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x04 \x81`@\x85\x01` \x87\x01a\x02\xF3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[a\x02\xB9\x80a\x04C`\09`\0\xF3\xFE`\x80`@R6a\0\x13Wa\0\x11a\0\x17V[\0[a\0\x11[a\0'a\0\"a\0gV[a\0\x9FV[V[``a\0N\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x02]`'\x919a\0\xC3V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[`\0a\0\x9A\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\0\xBEW=`\0\xF3[=`\0\xFD[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\0\xE0\x91\x90a\x02\rV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\x1BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01 V[``\x91P[P\x91P\x91Pa\x011\x86\x83\x83\x87a\x01;V[\x96\x95PPPPPPV[``\x83\x15a\x01\xADW\x82Q`\0\x03a\x01\xA6Wa\x01U\x85a\0UV[a\x01\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x81a\x01\xB7V[a\x01\xB7\x83\x83a\x01\xBFV[\x94\x93PPPPV[\x81Q\x15a\x01\xCFW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x9D\x91\x90a\x02)V[`\0[\x83\x81\x10\x15a\x02\x04W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xECV[PP`\0\x91\x01RV[`\0\x82Qa\x02\x1F\x81\x84` \x87\x01a\x01\xE9V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x02H\x81`@\x85\x01` \x87\x01a\x01\xE9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 '\x8F\\3\0^@\x8BN\xCDN<\xCA1\xA3\x96f\xA4\xAB*\x86\xA3\xE93\x0E\xDA\xAC\x7Fz\xAC\xC2\xB0dsolcC\0\x08\x11\x003Address: low-level delegate call failed\xA2dipfsX\"\x12 g\x12\x95:\xD4'\xEB\xD5\x02B\xF0^\xC6\xF8\xB5\x12\x1E1\xF6\xB8\x0B)!\x172O\x94\x13\x97\x98\xA5\rdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static ACCOUNTHANDLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x01\x1EW`\x005`\xE0\x1C\x80c\x01&-\xD2\x14b\0\x01#W\x80c\x17\xAC\xE6\xB5\x14b\0\x01RW\x80c+z\xC3\xF3\x14b\0\x01\x89W\x80c@\xD2\0A\x14b\0\x01\xB1W\x80cVd\xC7\x8E\x14b\0\x01\xCAW\x80c^_&\x10\x14b\0\x01\xE1W\x80ck\x0F\x04}\x14b\0\x02\tW\x80cm\xD5\x0F\x8B\x14b\0\x02\xA4W\x80cqP\x18\xA6\x14b\0\x02\xBBW\x80cr\x1CI\x96\x14b\0\x02\xC5W\x80c\x81\x17\xAB\xC1\x14b\0\x02\xDCW\x80c\x8D\xA5\xCB[\x14b\0\x03\x04W\x80c\xA9 \x14\xDD\x14b\0\x03\x0EW\x80c\xA9\x9Bj\xC6\x14b\0\x03KW\x80c\xAC\xAE\x05\xFE\x14b\0\x03wW\x80c\xB9\r6\xF9\x14b\0\x03\x8EW\x80c\xBAY\x1A6\x14b\0\x03\xB6W\x80c\xBFh\xC3\x06\x14b\0\x03\xD9W\x80c\xD3C\xD5\xCA\x14b\0\x04?W\x80c\xD8BHN\x14b\0\x04gW\x80c\xF2\xFD\xE3\x8B\x14b\0\x04~W[`\0\x80\xFD[b\0\x01:b\0\x0146`\x04b\0\x1A\xC4V[b\0\x04\x95V[`@Qb\0\x01I\x91\x90b\0\x1A\xDEV[`@Q\x80\x91\x03\x90\xF3[b\0\x01xb\0\x01c6`\x04b\0\x1A\xC4V[`\x05` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01b\0\x01IV[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x01\xC8b\0\x01\xC26`\x04b\0\x1B\x0FV[b\0\x04\xC3V[\0[b\0\x01:b\0\x01\xDB6`\x04b\0\x1A\xC4V[b\0\x04\xFBV[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x02vb\0\x02\x1A6`\x04b\0\x1A\xC4V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R\x93\x84R`\x03\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x85R`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15\x91\x84\x01\x91\x90\x91R`\x01\x01T\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x84\x01Q\x15\x15\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01b\0\x01IV[b\0\x01:b\0\x02\xB56`\x04b\0\x1B\x89V[b\0\x05\xBBV[b\0\x01\xC8b\0\n\x16V[b\0\x01\xC8b\0\x02\xD66`\x04b\0\x1C\xF1V[b\0\n.V[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x01:b\0\x12JV[b\0\x03<b\0\x03\x1F6`\x04b\0\x1E\x15V[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT\x81V[`@Q\x90\x81R` \x01b\0\x01IV[b\0\x01:b\0\x03\\6`\x04b\0\x1A\xC4V[`\x04` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[b\0\x01\xC8b\0\x03\x886`\x04b\0\x1EUV[b\0\x12YV[b\0\x03<\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x03<b\0\x03\xC76`\x04b\0\x1A\xC4V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[b\0\x04\x19b\0\x03\xEA6`\x04b\0\x1A\xC4V[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x82\x16\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90\x83V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x15\x15` \x84\x01R\x90\x82\x01R``\x01b\0\x01IV[b\0\x01:\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[b\0\x01xb\0\x04x6`\x04b\0\x1E\xF1V[b\0\x17\x88V[b\0\x01\xC8b\0\x04\x8F6`\x04b\0\x1FDV[b\0\x18FV[`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 T\x84R`\x03\x90\x91R\x82 \x01Tb\0\x04\xBD\x90b\0\x04\xFBV[\x92\x91PPV[b\0\x04\xCDb\0\x18\xC5V[`\0\x91\x82R`\x04` R`@\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0b\0\x04\xBD\x82`@Q\x80` \x01b\0\x05\x14\x90b\0\x1A\xB6V[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x04`\x1F\x90\x92\x01\x16\x90\x81R`$\x81\x01`@\x90\x81R` \x82\x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c J\x7F\x07`\xE2\x1B\x17\x90R\x90Qb\0\x05\x7F\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x90\x91\x01b\0\x1F\xB6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x05\x9F\x92\x91` \x01b\0\x1F\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 b\0\x19(V[`\0cei!\xFFB\x10b\0\x05\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 \x0FV[`@Q\x80\x91\x03\x90\xFD[`@Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\x06=\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\x81\x91\x90b\0 [V[\x03b\0\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 uV[`\0\x88\x81R`\x01` R`@\x90 T\x15b\0\x06\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmpointer exists`\x90\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x80\x1B`\x02\x86\x86`@Qb\0\x07\x08\x92\x91\x90b\0 \xA5V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14b\0\x07YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoPSI point exists`\x80\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x87\x81R`\x03` R`@\x90 `\x01\x01T\x15b\0\x07\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpwalletSalt exists`x\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[b\0\x07\xCCb\0\x07\xBD\x87b\0\x04\xFBV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x15b\0\x08\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x1D\xD8[\x1B\x19]\x08\x18[\x1C\x99XY\x1EH\x19\x19\\\x1B\x1B\xDEYY`J\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x99\x9A\xB5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c?V\x15\xA43`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x08\x92\x91\x90b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x08\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x08\xD6\x91\x90b\0 [V[\x8A\x8A\x8A\x8A\x8A\x8A\x8A`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\t\x02\x98\x97\x96\x95\x94\x93\x92\x91\x90b\0 \xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\t W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\tF\x91\x90b\0!0V[b\0\teW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!TV[`\0\x88\x81R`\x01` \x81\x81R`@\x80\x84 \x8B\x90U\x8A\x84R`\x03\x90\x91R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x81U\x01\x87\x90UQ\x88\x90`\x02\x90b\0\t\xAE\x90\x88\x90\x88\x90b\0 \xA5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 Ub\0\t\xC9\x86b\0\x19>V[\x90P\x85\x7F\xA6~\xDF\xB1WIs\xCC\x13\xEB\xF7\xC1x2\x8E\xC2\t|L\x16M\x95\x95\xC0\x06\xE6\\\x0F\xF0/\xBAf\x89\x89\x88\x88`@Qb\0\n\x03\x94\x93\x92\x91\x90b\0!\x8BV[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[b\0\n b\0\x18\xC5V[b\0\n,`\0b\0\x1A<V[V[`@Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\n\x7F\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\n\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\n\xC3\x91\x90b\0 [V[\x03b\0\n\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 uV[`\0\x86\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16b\0\x0BFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Ry\x1B\xDB\x19\x08\x1C\x99[\x18^Y\\\x88\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`2\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x86\x81R`\x03` R`@\x90 T3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03b\0\x0B\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01Rynew relayer cannot be same`0\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x86\x81R`\x03` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16b\0\x0C\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x18X\xD8\xDB\xDD[\x9D\x08\x1B\x9B\xDD\x08\x1A[\x9A]\x1AX[\x1A^\x99Y`J\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83` \x01Qb\0\x0CA\x91\x90b\0!\xB7V[\x11b\0\x0CaW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!\xD9V[`@\x80\x83\x01Q`\0\x90\x81R`\x05` R T`\xFF\x16\x15b\0\x0C\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0\"\0V[`\0\x85\x81R`\x01` R`@\x90 T\x15b\0\r$W`\0\x84\x81R`\x03` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15b\0\r\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fnew account is already initializ`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01b\0\x05\xE3V[b\0\r\x97V[`\0\x80\x1B`\x02\x84`@Qb\0\r:\x91\x90b\0\")V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14b\0\r\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{new PSI point already exists` \x1B`D\x82\x01R`d\x01b\0\x05\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x99\x9A\xB5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c?V\x15\xA43`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x0E\x14\x91\x90b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0EX\x91\x90b\0 [V[`\0\x89\x81R`\x03` R`@\x90\x81\x90 `\x01\x01T\x90Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Rb\0\x0E\x97\x92\x91\x8A\x91\x8A\x91\x90\x8A\x90\x89\x90`\x04\x01b\0\"GV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\xDB\x91\x90b\0!0V[b\0\x0E\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!TV[\x81Q``\x83\x01Q` \x80\x85\x01Q`@\x80\x87\x01Q`\0\x8C\x81R`\x03\x90\x94R\x92\x81\x90 T\x90Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x96c\x87\xFA\xC4\xBA\x96\x90\x95\x90\x94\x93\x90\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x92c?V\x15\xA4\x92b\0\x0F\x9C\x92\x91\x16\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0F\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0F\xE0\x91\x90b\0 [V[`@Qc\x0F\xD5\x85i`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\x10.\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x10LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10r\x91\x90b\0 [V[\x8D\x8C\x8B`\x80\x01Q`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x10\x9F\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90b\0\"\x95V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x10\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10\xE3\x91\x90b\0!0V[b\0\x111W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid account transport proof\0`D\x82\x01R`d\x01b\0\x05\xE3V[`@\x80\x83\x01Q`\0\x90\x81R`\x05` \x90\x81R\x82\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x88\x83R\x90R T\x15b\0\x11\x8DW`\0\x85\x81R`\x01` \x81\x81R`@\x80\x84 T\x84R`\x03\x90\x91R\x82 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U\x01U[`\0\x85\x81R`\x01` R`@\x90\x81\x90 \x85\x90UQ\x85\x90`\x02\x90b\0\x11\xB3\x90\x86\x90b\0\")V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 \x93\x90\x93U`\0\x89\x81R`\x03\x90\x92R\x80\x82 `\x01\x90\x81\x01T\x88\x84R\x91\x90\x92 \x91\x82\x01U\x80T`\x01`\xA0\x1B`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x163`\xFF`\xA0\x1B\x19\x16\x17\x17\x90U\x7F\xB0\xB7\xC3J<\xD8:\xFC<\xD4\xBC\xB5\xFD\xAFn\x9Fl\x94\xA5\xA5\xCCgV\x89\xED\x14G\xB2\xAE\xA9\xAC\x0E\x90b\0\x12:\x90\x88\x90\x88\x90\x88\x90\x88\x90b\0\"\xFCV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[cei!\xFFB\x10b\0\x12\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 \x0FV[`\0\x88\x81R`\x01` R`@\x80\x82 T\x90Qc\x0F\xD5\x85i`\xE2\x1B\x81R\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?V\x15\xA4\x90b\0\x12\xDF\x903\x90`\x04\x01b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x12\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x13#\x91\x90b\0 [V[\x03b\0\x13CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0 uV[\x80b\0\x13\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x18X\xD8\xDB\xDD[\x9D\x08\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`R\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x13\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn4\xB7;0\xB64\xB2\x1092\xB60\xBC\xB2\xB9`\x89\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15b\0\x14KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x18X\xD8\xDB\xDD[\x9D\x08\x18[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`*\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[`\0\x85\x81R`\x05` R`@\x90 T`\xFF\x16\x15b\0\x14}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0\"\0V[Bb\0\x14\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88b\0!\xB7V[\x11b\0\x14\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05\xE3\x90b\0!\xD9V[b\0\x15$`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pb\0\x17\x88\x91PPV[b\0\x15qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{\r-\xCE\xCC-\x8D,\x84\x08\x89i)\xA4\x0E\x0E\xACM\x8D,d\rl\xAF$\r\x0C.m`#\x1B`D\x82\x01R`d\x01b\0\x05\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCE.\x9F\x0E\x89\x89\x87\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c?V\x15\xA43`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x15\xF2\x91\x90b\0\x1A\xDEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x16\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x166\x91\x90b\0 [V[\x8F\x88\x8D\x8C\x8C`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x16b\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90b\0##V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x16\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x16\xA6\x91\x90b\0!0V[b\0\x17\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Finvalid account initialization p`D\x82\x01Rc97\xB7\xB3`\xE1\x1B`d\x82\x01R`\x84\x01b\0\x05\xE3V[`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x81U\x89\x85R`\x05\x83R\x81\x85 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x94\x86\x90R\x92\x82R\x92\x90\x91\x01T\x82Q\x8C\x81R\x91\x82\x01\x84\x90R\x91\x7F\x1A\xBA\x97\x02NW\x05T\x19\xDDN\x116\x1F\x9Ev\xF2\xBC\xEC\xF0\xE4\xD9\xA5\x1D\xE1\xC0\xE9\xDFfA\xD8\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPV[`\0\x83\x81R`\x04` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x17\xC9WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Qcs\xD3\xCB\xBD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xE7\xA7\x97z\x90b\0\x17\xF9\x90\x87\x90\x87\x90`\x04\x01b\0#\x86V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x18\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x18=\x91\x90b\0!0V[\x95\x94PPPPPV[b\0\x18Pb\0\x18\xC5V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x18\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x05\xE3V[b\0\x18\xC2\x81b\0\x1A<V[PV[3b\0\x18\xD0b\0\x12JV[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\n,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x05\xE3V[`\0b\0\x197\x83\x830b\0\x1A\x8CV[\x93\x92PPPV[`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c J\x7F\x07`\xE2\x1B\x17\x90R\x90Q`\0\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90b\0\x19\x9B\x90b\0\x1A\xB6V[b\0\x19\xA8\x92\x91\x90b\0\x1F\xB6V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x19\xC9W=`\0\x80>=`\0\xFD[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xFD\xE3\x8Bb\0\x19\xE5b\0\x12JV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x1A\x03\x91\x90b\0\x1A\xDEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1A\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1A3W=`\0\x80>=`\0\xFD[PPPP\x91\x90PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0`@Q\x83`@\x82\x01R\x84` \x82\x01R\x82\x81R`\x0B\x81\x01\x90P`\xFF\x81S`U\x90 \x94\x93PPPPV[a\x07#\x80b\0#\xAB\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x1A\xD7W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x1B\nW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x1B#W`\0\x80\xFD[\x825\x91Pb\0\x1B5` \x84\x01b\0\x1A\xF2V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0\x1BQW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x1BiW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15b\0\x1B\x82W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15b\0\x1B\xA5W`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1B\xD2W`\0\x80\xFD[b\0\x1B\xE0\x8B\x83\x8C\x01b\0\x1B>V[\x90\x96P\x94P`\x80\x8A\x015\x91P\x80\x82\x11\x15b\0\x1B\xFAW`\0\x80\xFD[Pb\0\x1C\t\x8A\x82\x8B\x01b\0\x1B>V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x1CWWb\0\x1CWb\0\x1C\x1CV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x1CoW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1C\x8CWb\0\x1C\x8Cb\0\x1C\x1CV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x1C\xB7Wb\0\x1C\xB7b\0\x1C\x1CV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x1C\xD1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x1D\x0BW`\0\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1D8W`\0\x80\xFD[b\0\x1DF\x8A\x83\x8B\x01b\0\x1C]V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15b\0\x1D]W`\0\x80\xFD[\x90\x88\x01\x90`\xA0\x82\x8B\x03\x12\x15b\0\x1DrW`\0\x80\xFD[b\0\x1D|b\0\x1C2V[\x825\x82\x81\x11\x15b\0\x1D\x8CW`\0\x80\xFD[b\0\x1D\x9A\x8C\x82\x86\x01b\0\x1C]V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15b\0\x1D\xCEW`\0\x80\xFD[b\0\x1D\xDC\x8C\x82\x86\x01b\0\x1C]V[`\x80\x83\x01RP\x93P`\xA0\x89\x015\x91P\x80\x82\x11\x15b\0\x1D\xF9W`\0\x80\xFD[Pb\0\x1E\x08\x89\x82\x8A\x01b\0\x1C]V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15b\0\x1E(W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x1E?W`\0\x80\xFD[b\0\x1EM\x84\x82\x85\x01b\0\x1C]V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15b\0\x1ErW`\0\x80\xFD[\x885\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x1E\x91W`\0\x80\xFD[b\0\x1E\x9F\x8C\x83\x8D\x01b\0\x1B>V[\x90\x99P\x97P`@\x8B\x015\x96P``\x8B\x015\x95P`\x80\x8B\x015\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15b\0\x1E\xCEW`\0\x80\xFD[Pb\0\x1E\xDD\x8B\x82\x8C\x01b\0\x1B>V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x1F\x07W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x1F%W`\0\x80\xFD[b\0\x1F3\x86\x82\x87\x01b\0\x1C]V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x1FWW`\0\x80\xFD[b\0\x197\x82b\0\x1A\xF2V[`\0[\x83\x81\x10\x15b\0\x1F\x7FW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x1FeV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Rb\0\x1F\xA2\x81` \x86\x01` \x86\x01b\0\x1FbV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90b\0\x1EM\x90\x83\x01\x84b\0\x1F\x88V[`\0\x83Qb\0\x1F\xF0\x81\x84` \x88\x01b\0\x1FbV[\x83Q\x90\x83\x01\x90b\0 \x06\x81\x83` \x88\x01b\0\x1FbV[\x01\x94\x93PPPPV[` \x80\x82R`,\x90\x82\x01R\x7Fthis function is not allowed fro`@\x82\x01Rkm 2023-12-01`\xA0\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0 nW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`\x16\x90\x82\x01Ru\x1C\x99[\x18^Y\\\x88\x1B\x9B\xDD\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`R\x1B`@\x82\x01R``\x01\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`\xC0`\x80\x82\x01R`\0b\0!\x0C`\xC0\x83\x01\x86\x88b\0 \xB5V[\x82\x81\x03`\xA0\x84\x01Rb\0!!\x81\x85\x87b\0 \xB5V[\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0!CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x197W`\0\x80\xFD[` \x80\x82R`\x1E\x90\x82\x01R\x7Finvalid account creation proof\0\0`@\x82\x01R``\x01\x90V[\x84\x81R\x83` \x82\x01R```@\x82\x01R`\0b\0!\xAD``\x83\x01\x84\x86b\0 \xB5V[\x96\x95PPPPPPV[\x80\x82\x01\x80\x82\x11\x15b\0\x04\xBDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[` \x80\x82R`\r\x90\x82\x01Rl\x19[XZ[\x08\x19^\x1C\x1A\\\x99Y`\x9A\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x0F\x90\x82\x01Rn\x19[XZ[\x08\x1B\x9D[\x1B\x1AY\x9AYY`\x8A\x1B`@\x82\x01R``\x01\x90V[`\0\x82Qb\0\"=\x81\x84` \x87\x01b\0\x1FbV[\x91\x90\x91\x01\x92\x91PPV[\x86\x81R\x85` \x82\x01R\x84`@\x82\x01R\x83``\x82\x01R`\xC0`\x80\x82\x01R`\0b\0\"t`\xC0\x83\x01\x85b\0\x1F\x88V[\x82\x81\x03`\xA0\x84\x01Rb\0\"\x88\x81\x85b\0\x1F\x88V[\x99\x98PPPPPPPPPV[`\0a\x01 \x80\x83Rb\0\"\xAB\x81\x84\x01\x8Db\0\x1F\x88V[\x90P\x8A` \x84\x01R\x89`@\x84\x01R\x88``\x84\x01R\x87`\x80\x84\x01R\x86`\xA0\x84\x01R\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Rb\0\"\xEC\x81\x85b\0\x1F\x88V[\x9C\x9BPPPPPPPPPPPPV[\x84\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0b\0!\xAD`\x80\x83\x01\x84b\0\x1F\x88V[`\0a\x01\0\x80\x83Rb\0#:\x81\x84\x01\x8D\x8Fb\0 \xB5V[\x90P\x8A` \x84\x01R\x89`@\x84\x01R\x88``\x84\x01R\x87`\x80\x84\x01R\x86`\xA0\x84\x01R\x85`\xC0\x84\x01R\x82\x81\x03`\xE0\x84\x01Rb\0#u\x81\x85\x87b\0 \xB5V[\x9D\x9CPPPPPPPPPPPPPV[`@\x81R`\0b\0#\x9B`@\x83\x01\x85b\0\x1F\x88V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE`\x80`@R`@Qa\x07#8\x03\x80a\x07#\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x03\x17V[a\0.\x82\x82`\0a\x005V[PPa\x044V[a\0>\x83a\0kV[`\0\x82Q\x11\x80a\0KWP\x80[\x15a\0fWa\0d\x83\x83a\0\xAB` \x1Ba\0)\x17` \x1CV[P[PPPV[a\0t\x81a\0\xD7V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\0\xD0\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x06\xFC`'\x919a\x01\xA9V[\x93\x92PPPV[a\0\xEA\x81a\x02\"` \x1Ba\0U\x17` \x1CV[a\x01QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x80a\x01\x88\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x021` \x1Ba\0d\x17` \x1CV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x01\xC6\x91\x90a\x03\xE5V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x02\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x06V[``\x91P[P\x90\x92P\x90Pa\x02\x18\x86\x83\x83\x87a\x024V[\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[``\x83\x15a\x02\xA1W\x82Q`\0\x03a\x02\x9AWa\x02N\x85a\x02\"V[a\x02\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01HV[P\x81a\x02\xABV[a\x02\xAB\x83\x83a\x02\xB3V[\x94\x93PPPPV[\x81Q\x15a\x02\xC3W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01H\x91\x90a\x04\x01V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x03\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xF6V[PP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x03*W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03AW`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x03^W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x03rW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x03\x84Wa\x03\x84a\x02\xDDV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x03\xACWa\x03\xACa\x02\xDDV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x03\xC5W`\0\x80\xFD[a\x03\xD6\x83` \x83\x01` \x88\x01a\x02\xF3V[\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x82Qa\x03\xF7\x81\x84` \x87\x01a\x02\xF3V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x04 \x81`@\x85\x01` \x87\x01a\x02\xF3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[a\x02\xB9\x80a\x04C`\09`\0\xF3\xFE`\x80`@R6a\0\x13Wa\0\x11a\0\x17V[\0[a\0\x11[a\0'a\0\"a\0gV[a\0\x9FV[V[``a\0N\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x02]`'\x919a\0\xC3V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[\x90V[`\0a\0\x9A\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\0\xBEW=`\0\xF3[=`\0\xFD[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\0\xE0\x91\x90a\x02\rV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\x1BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01 V[``\x91P[P\x91P\x91Pa\x011\x86\x83\x83\x87a\x01;V[\x96\x95PPPPPPV[``\x83\x15a\x01\xADW\x82Q`\0\x03a\x01\xA6Wa\x01U\x85a\0UV[a\x01\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x81a\x01\xB7V[a\x01\xB7\x83\x83a\x01\xBFV[\x94\x93PPPPV[\x81Q\x15a\x01\xCFW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x9D\x91\x90a\x02)V[`\0[\x83\x81\x10\x15a\x02\x04W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xECV[PP`\0\x91\x01RV[`\0\x82Qa\x02\x1F\x81\x84` \x87\x01a\x01\xE9V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x02H\x81`@\x85\x01` \x87\x01a\x01\xE9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 '\x8F\\3\0^@\x8BN\xCDN<\xCA1\xA3\x96f\xA4\xAB*\x86\xA3\xE93\x0E\xDA\xAC\x7Fz\xAC\xC2\xB0dsolcC\0\x08\x11\x003Address: low-level delegate call failed\xA2dipfsX\"\x12 g\x12\x95:\xD4'\xEB\xD5\x02B\xF0^\xC6\xF8\xB5\x12\x1E1\xF6\xB8\x0B)!\x172O\x94\x13\x97\x98\xA5\rdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static ACCOUNTHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AccountHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AccountHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AccountHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AccountHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AccountHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AccountHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AccountHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACCOUNTHANDLER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ACCOUNTHANDLER_ABI.clone(),
                ACCOUNTHANDLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `accountKeyCommitOfPointer` (0xba591a36) function
        pub fn account_key_commit_of_pointer(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 89, 26, 54], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAccount` (0x6dd50f8b) function
        pub fn create_account(
            &self,
            email_addr_pointer: [u8; 32],
            account_key_commit: [u8; 32],
            wallet_salt: [u8; 32],
            psi_point: ::ethers::core::types::Bytes,
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [109, 213, 15, 139],
                    (
                        email_addr_pointer,
                        account_key_commit,
                        wallet_salt,
                        psi_point,
                        proof,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultDkimRegistry` (0xd343d5ca) function
        pub fn default_dkim_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 67, 213, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dkimRegistryOfWalletSalt` (0xa99b6ac6) function
        pub fn dkim_registry_of_wallet_salt(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([169, 155, 106, 198], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emailNullifiers` (0x17ace6b5) function
        pub fn email_nullifiers(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 172, 230, 181], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emailValidityDuration` (0xb90d36f9) function
        pub fn email_validity_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 13, 54, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInfoOfAccountKeyCommit` (0x6b0f047d) function
        pub fn get_info_of_account_key_commit(
            &self,
            account_key_commit: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, AccountKeyInfo> {
            self.0
                .method_hash([107, 15, 4, 125], account_key_commit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWalletOfEmailAddrPointer` (0x01262dd2) function
        pub fn get_wallet_of_email_addr_pointer(
            &self,
            email_addr_pointer: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([1, 38, 45, 210], email_addr_pointer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWalletOfSalt` (0x5664c78e) function
        pub fn get_wallet_of_salt(
            &self,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([86, 100, 199, 142], salt)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `infoOfAccountKeyCommit` (0xbf68c306) function
        pub fn info_of_account_key_commit(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, bool, [u8; 32]),
        > {
            self.0
                .method_hash([191, 104, 195, 6], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeAccount` (0xacae05fe) function
        pub fn initialize_account(
            &self,
            email_addr_pointer: [u8; 32],
            email_domain: ::std::string::String,
            email_timestamp: ::ethers::core::types::U256,
            email_nullifier: [u8; 32],
            dkim_public_key_hash: [u8; 32],
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [172, 174, 5, 254],
                    (
                        email_addr_pointer,
                        email_domain,
                        email_timestamp,
                        email_nullifier,
                        dkim_public_key_hash,
                        proof,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0xd842484e) function
        pub fn is_dkim_public_key_hash_valid(
            &self,
            wallet_salt: [u8; 32],
            email_domain: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [216, 66, 72, 78],
                    (wallet_salt, email_domain, public_key_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pointerOfPSIPoint` (0xa92014dd) function
        pub fn pointer_of_psi_point(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([169, 32, 20, 221], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerHandler` (0x5e5f2610) function
        pub fn relayer_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([94, 95, 38, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transportAccount` (0x721c4996) function
        pub fn transport_account(
            &self,
            old_account_key_commit: [u8; 32],
            new_email_addr_pointer: [u8; 32],
            new_account_key_commit: [u8; 32],
            new_psi_point: ::ethers::core::types::Bytes,
            transport_email_proof: EmailProof,
            account_creation_proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [114, 28, 73, 150],
                    (
                        old_account_key_commit,
                        new_email_addr_pointer,
                        new_account_key_commit,
                        new_psi_point,
                        transport_email_proof,
                        account_creation_proof,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDKIMRegistryOfWalletSalt` (0x40d20041) function
        pub fn update_dkim_registry_of_wallet_salt(
            &self,
            wallet_salt: [u8; 32],
            dkim_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 210, 0, 65], (wallet_salt, dkim_registry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifier` (0x2b7ac3f3) function
        pub fn verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `walletImplementation` (0x8117abc1) function
        pub fn wallet_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([129, 23, 171, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AccountHandler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `accountKeyCommitOfPointer` function with signature `accountKeyCommitOfPointer(bytes32)` and selector `0xba591a36`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "accountKeyCommitOfPointer",
        abi = "accountKeyCommitOfPointer(bytes32)"
    )]
    pub struct AccountKeyCommitOfPointerCall(pub [u8; 32]);
    ///Container type for all input parameters for the `createAccount` function with signature `createAccount(bytes32,bytes32,bytes32,bytes,bytes)` and selector `0x6dd50f8b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createAccount",
        abi = "createAccount(bytes32,bytes32,bytes32,bytes,bytes)"
    )]
    pub struct CreateAccountCall {
        pub email_addr_pointer: [u8; 32],
        pub account_key_commit: [u8; 32],
        pub wallet_salt: [u8; 32],
        pub psi_point: ::ethers::core::types::Bytes,
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `defaultDkimRegistry` function with signature `defaultDkimRegistry()` and selector `0xd343d5ca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "defaultDkimRegistry", abi = "defaultDkimRegistry()")]
    pub struct DefaultDkimRegistryCall;
    ///Container type for all input parameters for the `dkimRegistryOfWalletSalt` function with signature `dkimRegistryOfWalletSalt(bytes32)` and selector `0xa99b6ac6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "dkimRegistryOfWalletSalt",
        abi = "dkimRegistryOfWalletSalt(bytes32)"
    )]
    pub struct DkimRegistryOfWalletSaltCall(pub [u8; 32]);
    ///Container type for all input parameters for the `emailNullifiers` function with signature `emailNullifiers(bytes32)` and selector `0x17ace6b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "emailNullifiers", abi = "emailNullifiers(bytes32)")]
    pub struct EmailNullifiersCall(pub [u8; 32]);
    ///Container type for all input parameters for the `emailValidityDuration` function with signature `emailValidityDuration()` and selector `0xb90d36f9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "emailValidityDuration", abi = "emailValidityDuration()")]
    pub struct EmailValidityDurationCall;
    ///Container type for all input parameters for the `getInfoOfAccountKeyCommit` function with signature `getInfoOfAccountKeyCommit(bytes32)` and selector `0x6b0f047d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getInfoOfAccountKeyCommit",
        abi = "getInfoOfAccountKeyCommit(bytes32)"
    )]
    pub struct GetInfoOfAccountKeyCommitCall {
        pub account_key_commit: [u8; 32],
    }
    ///Container type for all input parameters for the `getWalletOfEmailAddrPointer` function with signature `getWalletOfEmailAddrPointer(bytes32)` and selector `0x01262dd2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getWalletOfEmailAddrPointer",
        abi = "getWalletOfEmailAddrPointer(bytes32)"
    )]
    pub struct GetWalletOfEmailAddrPointerCall {
        pub email_addr_pointer: [u8; 32],
    }
    ///Container type for all input parameters for the `getWalletOfSalt` function with signature `getWalletOfSalt(bytes32)` and selector `0x5664c78e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getWalletOfSalt", abi = "getWalletOfSalt(bytes32)")]
    pub struct GetWalletOfSaltCall {
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `infoOfAccountKeyCommit` function with signature `infoOfAccountKeyCommit(bytes32)` and selector `0xbf68c306`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "infoOfAccountKeyCommit", abi = "infoOfAccountKeyCommit(bytes32)")]
    pub struct InfoOfAccountKeyCommitCall(pub [u8; 32]);
    ///Container type for all input parameters for the `initializeAccount` function with signature `initializeAccount(bytes32,string,uint256,bytes32,bytes32,bytes)` and selector `0xacae05fe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "initializeAccount",
        abi = "initializeAccount(bytes32,string,uint256,bytes32,bytes32,bytes)"
    )]
    pub struct InitializeAccountCall {
        pub email_addr_pointer: [u8; 32],
        pub email_domain: ::std::string::String,
        pub email_timestamp: ::ethers::core::types::U256,
        pub email_nullifier: [u8; 32],
        pub dkim_public_key_hash: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(bytes32,string,bytes32)` and selector `0xd842484e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isDKIMPublicKeyHashValid",
        abi = "isDKIMPublicKeyHashValid(bytes32,string,bytes32)"
    )]
    pub struct IsDKIMPublicKeyHashValidCall {
        pub wallet_salt: [u8; 32],
        pub email_domain: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pointerOfPSIPoint` function with signature `pointerOfPSIPoint(bytes)` and selector `0xa92014dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pointerOfPSIPoint", abi = "pointerOfPSIPoint(bytes)")]
    pub struct PointerOfPSIPointCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `relayerHandler` function with signature `relayerHandler()` and selector `0x5e5f2610`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "relayerHandler", abi = "relayerHandler()")]
    pub struct RelayerHandlerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transportAccount` function with signature `transportAccount(bytes32,bytes32,bytes32,bytes,(string,uint256,bytes32,bytes32,bytes),bytes)` and selector `0x721c4996`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "transportAccount",
        abi = "transportAccount(bytes32,bytes32,bytes32,bytes,(string,uint256,bytes32,bytes32,bytes),bytes)"
    )]
    pub struct TransportAccountCall {
        pub old_account_key_commit: [u8; 32],
        pub new_email_addr_pointer: [u8; 32],
        pub new_account_key_commit: [u8; 32],
        pub new_psi_point: ::ethers::core::types::Bytes,
        pub transport_email_proof: EmailProof,
        pub account_creation_proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateDKIMRegistryOfWalletSalt` function with signature `updateDKIMRegistryOfWalletSalt(bytes32,address)` and selector `0x40d20041`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updateDKIMRegistryOfWalletSalt",
        abi = "updateDKIMRegistryOfWalletSalt(bytes32,address)"
    )]
    pub struct UpdateDKIMRegistryOfWalletSaltCall {
        pub wallet_salt: [u8; 32],
        pub dkim_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
    ///Container type for all input parameters for the `walletImplementation` function with signature `walletImplementation()` and selector `0x8117abc1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "walletImplementation", abi = "walletImplementation()")]
    pub struct WalletImplementationCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccountHandlerCalls {
        AccountKeyCommitOfPointer(AccountKeyCommitOfPointerCall),
        CreateAccount(CreateAccountCall),
        DefaultDkimRegistry(DefaultDkimRegistryCall),
        DkimRegistryOfWalletSalt(DkimRegistryOfWalletSaltCall),
        EmailNullifiers(EmailNullifiersCall),
        EmailValidityDuration(EmailValidityDurationCall),
        GetInfoOfAccountKeyCommit(GetInfoOfAccountKeyCommitCall),
        GetWalletOfEmailAddrPointer(GetWalletOfEmailAddrPointerCall),
        GetWalletOfSalt(GetWalletOfSaltCall),
        InfoOfAccountKeyCommit(InfoOfAccountKeyCommitCall),
        InitializeAccount(InitializeAccountCall),
        IsDKIMPublicKeyHashValid(IsDKIMPublicKeyHashValidCall),
        Owner(OwnerCall),
        PointerOfPSIPoint(PointerOfPSIPointCall),
        RelayerHandler(RelayerHandlerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        TransportAccount(TransportAccountCall),
        UpdateDKIMRegistryOfWalletSalt(UpdateDKIMRegistryOfWalletSaltCall),
        Verifier(VerifierCall),
        WalletImplementation(WalletImplementationCall),
    }
    impl ::ethers::core::abi::AbiDecode for AccountHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountKeyCommitOfPointerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountKeyCommitOfPointer(decoded));
            }
            if let Ok(decoded) = <CreateAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateAccount(decoded));
            }
            if let Ok(decoded) = <DefaultDkimRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultDkimRegistry(decoded));
            }
            if let Ok(decoded) = <DkimRegistryOfWalletSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DkimRegistryOfWalletSalt(decoded));
            }
            if let Ok(decoded) = <EmailNullifiersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmailNullifiers(decoded));
            }
            if let Ok(decoded) = <EmailValidityDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmailValidityDuration(decoded));
            }
            if let Ok(decoded) = <GetInfoOfAccountKeyCommitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInfoOfAccountKeyCommit(decoded));
            }
            if let Ok(decoded) = <GetWalletOfEmailAddrPointerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWalletOfEmailAddrPointer(decoded));
            }
            if let Ok(decoded) = <GetWalletOfSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWalletOfSalt(decoded));
            }
            if let Ok(decoded) = <InfoOfAccountKeyCommitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InfoOfAccountKeyCommit(decoded));
            }
            if let Ok(decoded) = <InitializeAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeAccount(decoded));
            }
            if let Ok(decoded) = <IsDKIMPublicKeyHashValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDKIMPublicKeyHashValid(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PointerOfPSIPointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PointerOfPSIPoint(decoded));
            }
            if let Ok(decoded) = <RelayerHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayerHandler(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TransportAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransportAccount(decoded));
            }
            if let Ok(decoded) = <UpdateDKIMRegistryOfWalletSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDKIMRegistryOfWalletSalt(decoded));
            }
            if let Ok(decoded) = <VerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verifier(decoded));
            }
            if let Ok(decoded) = <WalletImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WalletImplementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccountHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountKeyCommitOfPointer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultDkimRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DkimRegistryOfWalletSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmailNullifiers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmailValidityDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInfoOfAccountKeyCommit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWalletOfEmailAddrPointer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWalletOfSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InfoOfAccountKeyCommit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PointerOfPSIPoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayerHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransportAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDKIMRegistryOfWalletSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WalletImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AccountHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountKeyCommitOfPointer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultDkimRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DkimRegistryOfWalletSalt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmailNullifiers(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmailValidityDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInfoOfAccountKeyCommit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetWalletOfEmailAddrPointer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetWalletOfSalt(element) => ::core::fmt::Display::fmt(element, f),
                Self::InfoOfAccountKeyCommit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PointerOfPSIPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransportAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDKIMRegistryOfWalletSalt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::WalletImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccountKeyCommitOfPointerCall> for AccountHandlerCalls {
        fn from(value: AccountKeyCommitOfPointerCall) -> Self {
            Self::AccountKeyCommitOfPointer(value)
        }
    }
    impl ::core::convert::From<CreateAccountCall> for AccountHandlerCalls {
        fn from(value: CreateAccountCall) -> Self {
            Self::CreateAccount(value)
        }
    }
    impl ::core::convert::From<DefaultDkimRegistryCall> for AccountHandlerCalls {
        fn from(value: DefaultDkimRegistryCall) -> Self {
            Self::DefaultDkimRegistry(value)
        }
    }
    impl ::core::convert::From<DkimRegistryOfWalletSaltCall> for AccountHandlerCalls {
        fn from(value: DkimRegistryOfWalletSaltCall) -> Self {
            Self::DkimRegistryOfWalletSalt(value)
        }
    }
    impl ::core::convert::From<EmailNullifiersCall> for AccountHandlerCalls {
        fn from(value: EmailNullifiersCall) -> Self {
            Self::EmailNullifiers(value)
        }
    }
    impl ::core::convert::From<EmailValidityDurationCall> for AccountHandlerCalls {
        fn from(value: EmailValidityDurationCall) -> Self {
            Self::EmailValidityDuration(value)
        }
    }
    impl ::core::convert::From<GetInfoOfAccountKeyCommitCall> for AccountHandlerCalls {
        fn from(value: GetInfoOfAccountKeyCommitCall) -> Self {
            Self::GetInfoOfAccountKeyCommit(value)
        }
    }
    impl ::core::convert::From<GetWalletOfEmailAddrPointerCall> for AccountHandlerCalls {
        fn from(value: GetWalletOfEmailAddrPointerCall) -> Self {
            Self::GetWalletOfEmailAddrPointer(value)
        }
    }
    impl ::core::convert::From<GetWalletOfSaltCall> for AccountHandlerCalls {
        fn from(value: GetWalletOfSaltCall) -> Self {
            Self::GetWalletOfSalt(value)
        }
    }
    impl ::core::convert::From<InfoOfAccountKeyCommitCall> for AccountHandlerCalls {
        fn from(value: InfoOfAccountKeyCommitCall) -> Self {
            Self::InfoOfAccountKeyCommit(value)
        }
    }
    impl ::core::convert::From<InitializeAccountCall> for AccountHandlerCalls {
        fn from(value: InitializeAccountCall) -> Self {
            Self::InitializeAccount(value)
        }
    }
    impl ::core::convert::From<IsDKIMPublicKeyHashValidCall> for AccountHandlerCalls {
        fn from(value: IsDKIMPublicKeyHashValidCall) -> Self {
            Self::IsDKIMPublicKeyHashValid(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AccountHandlerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PointerOfPSIPointCall> for AccountHandlerCalls {
        fn from(value: PointerOfPSIPointCall) -> Self {
            Self::PointerOfPSIPoint(value)
        }
    }
    impl ::core::convert::From<RelayerHandlerCall> for AccountHandlerCalls {
        fn from(value: RelayerHandlerCall) -> Self {
            Self::RelayerHandler(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AccountHandlerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AccountHandlerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransportAccountCall> for AccountHandlerCalls {
        fn from(value: TransportAccountCall) -> Self {
            Self::TransportAccount(value)
        }
    }
    impl ::core::convert::From<UpdateDKIMRegistryOfWalletSaltCall>
    for AccountHandlerCalls {
        fn from(value: UpdateDKIMRegistryOfWalletSaltCall) -> Self {
            Self::UpdateDKIMRegistryOfWalletSalt(value)
        }
    }
    impl ::core::convert::From<VerifierCall> for AccountHandlerCalls {
        fn from(value: VerifierCall) -> Self {
            Self::Verifier(value)
        }
    }
    impl ::core::convert::From<WalletImplementationCall> for AccountHandlerCalls {
        fn from(value: WalletImplementationCall) -> Self {
            Self::WalletImplementation(value)
        }
    }
    ///Container type for all return fields from the `accountKeyCommitOfPointer` function with signature `accountKeyCommitOfPointer(bytes32)` and selector `0xba591a36`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccountKeyCommitOfPointerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `createAccount` function with signature `createAccount(bytes32,bytes32,bytes32,bytes,bytes)` and selector `0x6dd50f8b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateAccountReturn {
        pub wallet: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `defaultDkimRegistry` function with signature `defaultDkimRegistry()` and selector `0xd343d5ca`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultDkimRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `dkimRegistryOfWalletSalt` function with signature `dkimRegistryOfWalletSalt(bytes32)` and selector `0xa99b6ac6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DkimRegistryOfWalletSaltReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `emailNullifiers` function with signature `emailNullifiers(bytes32)` and selector `0x17ace6b5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EmailNullifiersReturn(pub bool);
    ///Container type for all return fields from the `emailValidityDuration` function with signature `emailValidityDuration()` and selector `0xb90d36f9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EmailValidityDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getInfoOfAccountKeyCommit` function with signature `getInfoOfAccountKeyCommit(bytes32)` and selector `0x6b0f047d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetInfoOfAccountKeyCommitReturn(pub AccountKeyInfo);
    ///Container type for all return fields from the `getWalletOfEmailAddrPointer` function with signature `getWalletOfEmailAddrPointer(bytes32)` and selector `0x01262dd2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetWalletOfEmailAddrPointerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getWalletOfSalt` function with signature `getWalletOfSalt(bytes32)` and selector `0x5664c78e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetWalletOfSaltReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `infoOfAccountKeyCommit` function with signature `infoOfAccountKeyCommit(bytes32)` and selector `0xbf68c306`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct InfoOfAccountKeyCommitReturn {
        pub relayer: ::ethers::core::types::Address,
        pub initialized: bool,
        pub wallet_salt: [u8; 32],
    }
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(bytes32,string,bytes32)` and selector `0xd842484e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsDKIMPublicKeyHashValidReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pointerOfPSIPoint` function with signature `pointerOfPSIPoint(bytes)` and selector `0xa92014dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PointerOfPSIPointReturn(pub [u8; 32]);
    ///Container type for all return fields from the `relayerHandler` function with signature `relayerHandler()` and selector `0x5e5f2610`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RelayerHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VerifierReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `walletImplementation` function with signature `walletImplementation()` and selector `0x8117abc1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WalletImplementationReturn(pub ::ethers::core::types::Address);
    ///`AccountKeyInfo(address,bool,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccountKeyInfo {
        pub relayer: ::ethers::core::types::Address,
        pub initialized: bool,
        pub wallet_salt: [u8; 32],
    }
    ///`EmailProof(string,uint256,bytes32,bytes32,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EmailProof {
        pub domain: ::std::string::String,
        pub timestamp: ::ethers::core::types::U256,
        pub nullifier: [u8; 32],
        pub dkim_public_key_hash: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
}
