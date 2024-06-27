pub use liquidator_paraswap::*;
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
pub mod liquidator_paraswap {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AUGUSTUS_REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AUGUSTUS_REGISTRY"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAugustusRegistry",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeOperation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("premiums"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ILiquidatorParaswap.LiquidationParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("flashLoanPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFlashLoan"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ILiquidatorParaswap.SwapParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedLiquidationCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailedLiquidationCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAugustusInstance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidAugustusInstance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDebtBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidDebtBalance"),
                            inputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitiator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidInitiator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SenderNotPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SenderNotPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATORPARASWAP_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P3\x80a\x007W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0@\x81a\0FV[Pa\0\x96V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x15O\x80a\0\xA5`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xDFW\x80c\x90<\xA2\x17\x14a\0\xF0W\x80c\x92\x0F\\\x84\x14a\x01\x18W\x80c\xF2\xFD\xE3\x8B\x14a\x01;W`\0\x80\xFD[\x80c:\x82\x98g\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\xBAW\x80cu5\xD2F\x14a\0\xC4W[`\0\x80\xFD[a\0\x9Ds~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC2a\x01NV[\0[a\0\x9Ds\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\x9DV[a\x01\x03a\0\xFE6`\x04a\x0E_V[a\x01bV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB1V[a\x01+a\x01&6`\x04a\x0F2V[a\x04\xDDV[`@Q\x90\x15\x15\x81R` \x01a\0\xB1V[a\0\xC2a\x01I6`\x04a\x107V[a\x07\x15V[a\x01Va\x07SV[a\x01``\0a\x07\x80V[V[`\0\x80a\x01ma\x07SV[`\0a\x01\x7F`@\x87\x01` \x88\x01a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE9\x91\x90a\x10TV[\x90P`\0a\x01\xFA` \x88\x01\x88a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90a\x10TV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x90\x91U\x90\x91Pc\xAB\x9CK]0a\x02\xA3a\x02\x9E`@\x8C\x01` \x8D\x01a\x107V[a\x07\xD0V[a\x02\xB0\x8B`@\x015a\x08+V[a\x02\xBA`\0a\x08+V[0`@Q\x80`\x80\x01`@R\x80\x8A\x81R` \x01\x8F`\0\x01` \x81\x01\x90a\x02\xDF\x91\x90a\x107V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F\x80``\x01\x90a\x02\xFD\x91\x90a\x10mV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x03@\x8Ea\x11\xFAV[\x90R`@Qa\x03R\x91\x90` \x01a\x12VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\0`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x85\x97\x96\x95\x94\x93\x92\x91\x90a\x13\x02V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x9FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xB3W=`\0\x80>=`\0\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP\x81\x90Pa\x03\xD6` \x89\x01\x89a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04@\x91\x90a\x10TV[a\x04J\x91\x90a\x13\xD4V[\x93P\x81a\x04]`@\x89\x01` \x8A\x01a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC7\x91\x90a\x10TV[a\x04\xD1\x91\x90a\x13\xD4V[\x92PPP\x93P\x93\x91PPV[`\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x12W`@Qc\x84qa\x8B`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x160\x14a\x05FW`@Qc\x02\xA8h\xED`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05\tV[`\0a\x05T\x83\x85\x01\x85a\x13\xE7V[\x90Pa\x05\xBC\x81``\x01Q`\0\x01Q\x8C\x8C`\0\x81\x81\x10a\x05uWa\x05ua\x14\x95V[\x90P` \x02\x01` \x81\x01\x90a\x05\x8A\x91\x90a\x107V[\x83` \x01Q\x8C\x8C`\0\x81\x81\x10a\x05\xA2Wa\x05\xA2a\x14\x95V[\x90P` \x02\x015\x85`@\x01Q\x86``\x01Q` \x01Qa\x08rV[`\0\x87\x87`\0\x81\x81\x10a\x05\xD1Wa\x05\xD1a\x14\x95V[\x90P` \x02\x015\x8A\x8A`\0\x81\x81\x10a\x05\xEBWa\x05\xEBa\x14\x95V[\x90P` \x02\x015a\x05\xFC\x91\x90a\x14\xABV[\x90P`\0\x81\x8D\x8D`\0\x81\x81\x10a\x06\x14Wa\x06\x14a\x14\x95V[\x90P` \x02\x01` \x81\x01\x90a\x06)\x91\x90a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90a\x10TV[a\x06\x9D\x91\x90a\x13\xD4V[\x83Q\x90\x91P\x81\x10\x15a\x06\xC5W`@Qc\x0C\x1D\xE0\xD3`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05\tV[a\x07\x02\x8D\x8D`\0\x81\x81\x10a\x06\xDBWa\x06\xDBa\x14\x95V[\x90P` \x02\x01` \x81\x01\x90a\x06\xF0\x91\x90a\x107V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\x0BEV[P`\x01\x9C\x9BPPPPPPPPPPPPV[a\x07\x1Da\x07SV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07GW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05\tV[a\x07P\x81a\x07\x80V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01`W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x05\tV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x08\x06Wa\x08\x06a\x14\x95V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x08aWa\x08aa\x14\x95V[` \x02` \x01\x01\x81\x81RPP\x91\x90PV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDD\x91\x90a\x10TV[\x90Pa\x08\xFE\x86s\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x86a\x0BEV[`\0s\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\t,\x91\x90a\x14\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\tiW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tnV[``\x91P[PP\x90P\x80a\t\x93W`@Q`\x01b\x15\x9E\x13`\xE1\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\0\x91\x90a\x10TV[a\n\n\x91\x90a\x13\xD4V[`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01R\x90\x91Ps~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\neW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x89\x91\x90a\x14\xDAV[a\n\xB1W`@Qc[\x15\x99\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x01a\x05\tV[`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x15\x91\x90a\x14\xFCV[\x90Pa\x0B\"\x88\x82\x84a\x0BEV[a\x0B,\x8A\x86a\x0B\xFFV[Pa\x0B9\x88\x82`\0a\x0BEV[PPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0B\x96\x84\x82a\x0C\x16V[a\x0B\xF9W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0B\xEF\x90\x85\x90a\x0C\xBEV[a\x0B\xF9\x84\x82a\x0C\xBEV[PPPPV[``a\x0C\r\x83\x83`\0a\r&V[\x90P[\x92\x91PPV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0C3\x91\x90a\x14\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0CpW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CuV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0C\x9FWP\x80Q\x15\x80a\x0C\x9FWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\x9F\x91\x90a\x14\xDAV[\x80\x15a\x0C\xB5WP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\x0C\xD3`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x0B\xFFV[\x90P\x80Q`\0\x14\x15\x80\x15a\x0C\xF8WP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\xF6\x91\x90a\x14\xDAV[\x15[\x15a\r!W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x05\tV[PPPV[``\x81G\x10\x15a\rKW`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x05\tV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\rg\x91\x90a\x14\xBEV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\r\xA4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xA9V[``\x91P[P\x91P\x91Pa\r\xB9\x86\x83\x83a\r\xC5V[\x92PPP[\x93\x92PPPV[``\x82a\r\xDAWa\r\xD5\x82a\x0E!V[a\r\xBEV[\x81Q\x15\x80\x15a\r\xF1WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0E\x1AW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05\tV[P\x80a\r\xBEV[\x80Q\x15a\x0E1W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07PW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0EtW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x8CW`\0\x80\xFD[\x90\x85\x01\x90`\x80\x82\x88\x03\x12\x15a\x0E\xA0W`\0\x80\xFD[\x90\x93P` \x85\x015\x90a\x0E\xB2\x82a\x0EJV[\x90\x92P`@\x85\x015\x90\x80\x82\x11\x15a\x0E\xC8W`\0\x80\xFD[P\x84\x01`@\x81\x87\x03\x12\x15a\x0E\xDBW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\0\x80\x83`\x1F\x84\x01\x12a\x0E\xF8W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x10W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0F+W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\x0FPW`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FhW`\0\x80\xFD[a\x0Ft\x8D\x83\x8E\x01a\x0E\xE6V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x0F\x8DW`\0\x80\xFD[a\x0F\x99\x8D\x83\x8E\x01a\x0E\xE6V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x0F\xB2W`\0\x80\xFD[a\x0F\xBE\x8D\x83\x8E\x01a\x0E\xE6V[\x90\x97P\x95P``\x8C\x015\x91Pa\x0F\xD3\x82a\x0EJV[\x90\x93P`\x80\x8B\x015\x90\x80\x82\x11\x15a\x0F\xE9W`\0\x80\xFD[\x81\x8C\x01\x91P\x8C`\x1F\x83\x01\x12a\x0F\xFDW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10\x0CW`\0\x80\xFD[\x8D` \x82\x85\x01\x01\x11\x15a\x10\x1EW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0` \x82\x84\x03\x12\x15a\x10IW`\0\x80\xFD[\x815a\r\xBE\x81a\x0EJV[`\0` \x82\x84\x03\x12\x15a\x10fW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x10\x84W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\x9FW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0F+W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xEDWa\x10\xEDa\x10\xB4V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x11\x04W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\x1FWa\x11\x1Fa\x10\xB4V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x11GWa\x11Ga\x10\xB4V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x11`W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x11\x92W`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x11\xB6Wa\x11\xB6a\x10\xB4V[\x81`@R\x82\x93P\x845\x91Pa\x11\xCA\x82a\x0EJV[\x90\x82R` \x84\x015\x90\x80\x82\x11\x15a\x11\xE0W`\0\x80\xFD[Pa\x11\xED\x85\x82\x86\x01a\x10\xF3V[` \x83\x01RPP\x92\x91PPV[`\0a\x0C\x106\x83a\x11\x80V[`\0[\x83\x81\x10\x15a\x12!W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\tV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12B\x81` \x86\x01` \x86\x01a\x12\x06V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q`\x80``\x84\x01R`\0\x91a\x12\x92`\xA0\x85\x01\x83a\x12*V[\x91P``\x85\x01Q`\x1F\x19\x85\x84\x03\x01`\x80\x86\x01R\x81\x81Q\x16\x83R` \x81\x01Q\x91PP`@` \x83\x01Ra\x0C\xB5`@\x83\x01\x82a\x12*V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x12\xF7W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x12\xDBV[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x13QW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x133V[PP\x85\x81\x03`@\x87\x01Ra\x13e\x81\x8Ca\x12\xC7V[\x93PPPP\x82\x81\x03``\x84\x01Ra\x13|\x81\x88a\x12\xC7V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x13\x9F\x81\x86a\x12*V[\x91PPa\x13\xB2`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\x10Wa\x0C\x10a\x13\xBEV[`\0` \x82\x84\x03\x12\x15a\x13\xF9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\x11W`\0\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15a\x14%W`\0\x80\xFD[a\x14-a\x10\xCAV[\x825\x81R` \x83\x015a\x14?\x81a\x0EJV[` \x82\x01R`@\x83\x015\x82\x81\x11\x15a\x14VW`\0\x80\xFD[a\x14b\x87\x82\x86\x01a\x10\xF3V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x14zW`\0\x80\xFD[a\x14\x86\x87\x82\x86\x01a\x11\x80V[``\x83\x01RP\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\x10Wa\x0C\x10a\x13\xBEV[`\0\x82Qa\x14\xD0\x81\x84` \x87\x01a\x12\x06V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\xECW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\r\xBEW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x15\x0EW`\0\x80\xFD[\x81Qa\r\xBE\x81a\x0EJV\xFE\xA2dipfsX\"\x12 \n\x87\x9C\x85h\xCC\x81\x92\x88=\x8B\xF3E\xE4$\xDD\xB6X\xCE\x9E\xFEvu\x9F\xA5\xC6Bn,p\xFF\xDDdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATORPARASWAP_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xDFW\x80c\x90<\xA2\x17\x14a\0\xF0W\x80c\x92\x0F\\\x84\x14a\x01\x18W\x80c\xF2\xFD\xE3\x8B\x14a\x01;W`\0\x80\xFD[\x80c:\x82\x98g\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\xBAW\x80cu5\xD2F\x14a\0\xC4W[`\0\x80\xFD[a\0\x9Ds~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC2a\x01NV[\0[a\0\x9Ds\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\x9DV[a\x01\x03a\0\xFE6`\x04a\x0E_V[a\x01bV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB1V[a\x01+a\x01&6`\x04a\x0F2V[a\x04\xDDV[`@Q\x90\x15\x15\x81R` \x01a\0\xB1V[a\0\xC2a\x01I6`\x04a\x107V[a\x07\x15V[a\x01Va\x07SV[a\x01``\0a\x07\x80V[V[`\0\x80a\x01ma\x07SV[`\0a\x01\x7F`@\x87\x01` \x88\x01a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE9\x91\x90a\x10TV[\x90P`\0a\x01\xFA` \x88\x01\x88a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90a\x10TV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x90\x91U\x90\x91Pc\xAB\x9CK]0a\x02\xA3a\x02\x9E`@\x8C\x01` \x8D\x01a\x107V[a\x07\xD0V[a\x02\xB0\x8B`@\x015a\x08+V[a\x02\xBA`\0a\x08+V[0`@Q\x80`\x80\x01`@R\x80\x8A\x81R` \x01\x8F`\0\x01` \x81\x01\x90a\x02\xDF\x91\x90a\x107V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F\x80``\x01\x90a\x02\xFD\x91\x90a\x10mV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x03@\x8Ea\x11\xFAV[\x90R`@Qa\x03R\x91\x90` \x01a\x12VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\0`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x85\x97\x96\x95\x94\x93\x92\x91\x90a\x13\x02V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x9FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xB3W=`\0\x80>=`\0\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP\x81\x90Pa\x03\xD6` \x89\x01\x89a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04@\x91\x90a\x10TV[a\x04J\x91\x90a\x13\xD4V[\x93P\x81a\x04]`@\x89\x01` \x8A\x01a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC7\x91\x90a\x10TV[a\x04\xD1\x91\x90a\x13\xD4V[\x92PPP\x93P\x93\x91PPV[`\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x12W`@Qc\x84qa\x8B`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x160\x14a\x05FW`@Qc\x02\xA8h\xED`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05\tV[`\0a\x05T\x83\x85\x01\x85a\x13\xE7V[\x90Pa\x05\xBC\x81``\x01Q`\0\x01Q\x8C\x8C`\0\x81\x81\x10a\x05uWa\x05ua\x14\x95V[\x90P` \x02\x01` \x81\x01\x90a\x05\x8A\x91\x90a\x107V[\x83` \x01Q\x8C\x8C`\0\x81\x81\x10a\x05\xA2Wa\x05\xA2a\x14\x95V[\x90P` \x02\x015\x85`@\x01Q\x86``\x01Q` \x01Qa\x08rV[`\0\x87\x87`\0\x81\x81\x10a\x05\xD1Wa\x05\xD1a\x14\x95V[\x90P` \x02\x015\x8A\x8A`\0\x81\x81\x10a\x05\xEBWa\x05\xEBa\x14\x95V[\x90P` \x02\x015a\x05\xFC\x91\x90a\x14\xABV[\x90P`\0\x81\x8D\x8D`\0\x81\x81\x10a\x06\x14Wa\x06\x14a\x14\x95V[\x90P` \x02\x01` \x81\x01\x90a\x06)\x91\x90a\x107V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90a\x10TV[a\x06\x9D\x91\x90a\x13\xD4V[\x83Q\x90\x91P\x81\x10\x15a\x06\xC5W`@Qc\x0C\x1D\xE0\xD3`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05\tV[a\x07\x02\x8D\x8D`\0\x81\x81\x10a\x06\xDBWa\x06\xDBa\x14\x95V[\x90P` \x02\x01` \x81\x01\x90a\x06\xF0\x91\x90a\x107V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\x0BEV[P`\x01\x9C\x9BPPPPPPPPPPPPV[a\x07\x1Da\x07SV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07GW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05\tV[a\x07P\x81a\x07\x80V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01`W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x05\tV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x08\x06Wa\x08\x06a\x14\x95V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x08aWa\x08aa\x14\x95V[` \x02` \x01\x01\x81\x81RPP\x91\x90PV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDD\x91\x90a\x10TV[\x90Pa\x08\xFE\x86s\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x86a\x0BEV[`\0s\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\t,\x91\x90a\x14\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\tiW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tnV[``\x91P[PP\x90P\x80a\t\x93W`@Q`\x01b\x15\x9E\x13`\xE1\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\0\x91\x90a\x10TV[a\n\n\x91\x90a\x13\xD4V[`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01R\x90\x91Ps~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\neW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x89\x91\x90a\x14\xDAV[a\n\xB1W`@Qc[\x15\x99\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x01a\x05\tV[`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x15\x91\x90a\x14\xFCV[\x90Pa\x0B\"\x88\x82\x84a\x0BEV[a\x0B,\x8A\x86a\x0B\xFFV[Pa\x0B9\x88\x82`\0a\x0BEV[PPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0B\x96\x84\x82a\x0C\x16V[a\x0B\xF9W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0B\xEF\x90\x85\x90a\x0C\xBEV[a\x0B\xF9\x84\x82a\x0C\xBEV[PPPPV[``a\x0C\r\x83\x83`\0a\r&V[\x90P[\x92\x91PPV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0C3\x91\x90a\x14\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0CpW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CuV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0C\x9FWP\x80Q\x15\x80a\x0C\x9FWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\x9F\x91\x90a\x14\xDAV[\x80\x15a\x0C\xB5WP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\x0C\xD3`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x0B\xFFV[\x90P\x80Q`\0\x14\x15\x80\x15a\x0C\xF8WP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\xF6\x91\x90a\x14\xDAV[\x15[\x15a\r!W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x05\tV[PPPV[``\x81G\x10\x15a\rKW`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x05\tV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\rg\x91\x90a\x14\xBEV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\r\xA4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xA9V[``\x91P[P\x91P\x91Pa\r\xB9\x86\x83\x83a\r\xC5V[\x92PPP[\x93\x92PPPV[``\x82a\r\xDAWa\r\xD5\x82a\x0E!V[a\r\xBEV[\x81Q\x15\x80\x15a\r\xF1WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0E\x1AW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05\tV[P\x80a\r\xBEV[\x80Q\x15a\x0E1W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07PW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0EtW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x8CW`\0\x80\xFD[\x90\x85\x01\x90`\x80\x82\x88\x03\x12\x15a\x0E\xA0W`\0\x80\xFD[\x90\x93P` \x85\x015\x90a\x0E\xB2\x82a\x0EJV[\x90\x92P`@\x85\x015\x90\x80\x82\x11\x15a\x0E\xC8W`\0\x80\xFD[P\x84\x01`@\x81\x87\x03\x12\x15a\x0E\xDBW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\0\x80\x83`\x1F\x84\x01\x12a\x0E\xF8W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x10W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0F+W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\x0FPW`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FhW`\0\x80\xFD[a\x0Ft\x8D\x83\x8E\x01a\x0E\xE6V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x0F\x8DW`\0\x80\xFD[a\x0F\x99\x8D\x83\x8E\x01a\x0E\xE6V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x0F\xB2W`\0\x80\xFD[a\x0F\xBE\x8D\x83\x8E\x01a\x0E\xE6V[\x90\x97P\x95P``\x8C\x015\x91Pa\x0F\xD3\x82a\x0EJV[\x90\x93P`\x80\x8B\x015\x90\x80\x82\x11\x15a\x0F\xE9W`\0\x80\xFD[\x81\x8C\x01\x91P\x8C`\x1F\x83\x01\x12a\x0F\xFDW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10\x0CW`\0\x80\xFD[\x8D` \x82\x85\x01\x01\x11\x15a\x10\x1EW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0` \x82\x84\x03\x12\x15a\x10IW`\0\x80\xFD[\x815a\r\xBE\x81a\x0EJV[`\0` \x82\x84\x03\x12\x15a\x10fW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x10\x84W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\x9FW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0F+W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xEDWa\x10\xEDa\x10\xB4V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x11\x04W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\x1FWa\x11\x1Fa\x10\xB4V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x11GWa\x11Ga\x10\xB4V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x11`W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x11\x92W`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x11\xB6Wa\x11\xB6a\x10\xB4V[\x81`@R\x82\x93P\x845\x91Pa\x11\xCA\x82a\x0EJV[\x90\x82R` \x84\x015\x90\x80\x82\x11\x15a\x11\xE0W`\0\x80\xFD[Pa\x11\xED\x85\x82\x86\x01a\x10\xF3V[` \x83\x01RPP\x92\x91PPV[`\0a\x0C\x106\x83a\x11\x80V[`\0[\x83\x81\x10\x15a\x12!W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\tV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12B\x81` \x86\x01` \x86\x01a\x12\x06V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q`\x80``\x84\x01R`\0\x91a\x12\x92`\xA0\x85\x01\x83a\x12*V[\x91P``\x85\x01Q`\x1F\x19\x85\x84\x03\x01`\x80\x86\x01R\x81\x81Q\x16\x83R` \x81\x01Q\x91PP`@` \x83\x01Ra\x0C\xB5`@\x83\x01\x82a\x12*V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x12\xF7W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x12\xDBV[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x13QW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x133V[PP\x85\x81\x03`@\x87\x01Ra\x13e\x81\x8Ca\x12\xC7V[\x93PPPP\x82\x81\x03``\x84\x01Ra\x13|\x81\x88a\x12\xC7V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x13\x9F\x81\x86a\x12*V[\x91PPa\x13\xB2`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\x10Wa\x0C\x10a\x13\xBEV[`\0` \x82\x84\x03\x12\x15a\x13\xF9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\x11W`\0\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15a\x14%W`\0\x80\xFD[a\x14-a\x10\xCAV[\x825\x81R` \x83\x015a\x14?\x81a\x0EJV[` \x82\x01R`@\x83\x015\x82\x81\x11\x15a\x14VW`\0\x80\xFD[a\x14b\x87\x82\x86\x01a\x10\xF3V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x14zW`\0\x80\xFD[a\x14\x86\x87\x82\x86\x01a\x11\x80V[``\x83\x01RP\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\x10Wa\x0C\x10a\x13\xBEV[`\0\x82Qa\x14\xD0\x81\x84` \x87\x01a\x12\x06V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\xECW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\r\xBEW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x15\x0EW`\0\x80\xFD[\x81Qa\r\xBE\x81a\x0EJV\xFE\xA2dipfsX\"\x12 \n\x87\x9C\x85h\xCC\x81\x92\x88=\x8B\xF3E\xE4$\xDD\xB6X\xCE\x9E\xFEvu\x9F\xA5\xC6Bn,p\xFF\xDDdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATORPARASWAP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LiquidatorParaswap<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LiquidatorParaswap<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LiquidatorParaswap<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LiquidatorParaswap<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LiquidatorParaswap<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LiquidatorParaswap))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LiquidatorParaswap<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATORPARASWAP_ABI.clone(),
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
                LIQUIDATORPARASWAP_ABI.clone(),
                LIQUIDATORPARASWAP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `AUGUSTUS_REGISTRY` (0x3a829867) function
        pub fn augustus_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([58, 130, 152, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL` (0x7535d246) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeOperation` (0x920f5c84) function
        pub fn execute_operation(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            premiums: ::std::vec::Vec<::ethers::core::types::U256>,
            initiator: ::ethers::core::types::Address,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [146, 15, 92, 132],
                    (assets, amounts, premiums, initiator, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x903ca217) function
        pub fn liquidate(
            &self,
            liquidation_params: LiquidationParams,
            flash_loan_pool: ::ethers::core::types::Address,
            swap_params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [144, 60, 162, 23],
                    (liquidation_params, flash_loan_pool, swap_params),
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
    for LiquidatorParaswap<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `FailedLiquidationCall` with signature `FailedLiquidationCall()` and selector `0xffd4c3da`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedLiquidationCall", abi = "FailedLiquidationCall()")]
    pub struct FailedLiquidationCall;
    ///Custom Error type `InvalidAugustusInstance` with signature `InvalidAugustusInstance(address)` and selector `0x5b159995`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidAugustusInstance",
        abi = "InvalidAugustusInstance(address)"
    )]
    pub struct InvalidAugustusInstance(pub ::ethers::core::types::Address);
    ///Custom Error type `InvalidDebtBalance` with signature `InvalidDebtBalance(uint256)` and selector `0x60ef0698`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidDebtBalance", abi = "InvalidDebtBalance(uint256)")]
    pub struct InvalidDebtBalance(pub ::ethers::core::types::U256);
    ///Custom Error type `InvalidInitiator` with signature `InvalidInitiator(address)` and selector `0x2a868ed0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidInitiator", abi = "InvalidInitiator(address)")]
    pub struct InvalidInitiator(pub ::ethers::core::types::Address);
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `SenderNotPool` with signature `SenderNotPool(address)` and selector `0x8471618b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SenderNotPool", abi = "SenderNotPool(address)")]
    pub struct SenderNotPool(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LiquidatorParaswapErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        FailedLiquidationCall(FailedLiquidationCall),
        InvalidAugustusInstance(InvalidAugustusInstance),
        InvalidDebtBalance(InvalidDebtBalance),
        InvalidInitiator(InvalidInitiator),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        SenderNotPool(SenderNotPool),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorParaswapErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <FailedLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedLiquidationCall(decoded));
            }
            if let Ok(decoded) = <InvalidAugustusInstance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAugustusInstance(decoded));
            }
            if let Ok(decoded) = <InvalidDebtBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDebtBalance(decoded));
            }
            if let Ok(decoded) = <InvalidInitiator as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInitiator(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <SenderNotPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SenderNotPool(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorParaswapErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedLiquidationCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAugustusInstance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDebtBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitiator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SenderNotPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LiquidatorParaswapErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedLiquidationCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAugustusInstance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDebtBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitiator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SenderNotPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorParaswapErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedLiquidationCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAugustusInstance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDebtBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInitiator(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SenderNotPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidatorParaswapErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LiquidatorParaswapErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for LiquidatorParaswapErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LiquidatorParaswapErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<FailedLiquidationCall> for LiquidatorParaswapErrors {
        fn from(value: FailedLiquidationCall) -> Self {
            Self::FailedLiquidationCall(value)
        }
    }
    impl ::core::convert::From<InvalidAugustusInstance> for LiquidatorParaswapErrors {
        fn from(value: InvalidAugustusInstance) -> Self {
            Self::InvalidAugustusInstance(value)
        }
    }
    impl ::core::convert::From<InvalidDebtBalance> for LiquidatorParaswapErrors {
        fn from(value: InvalidDebtBalance) -> Self {
            Self::InvalidDebtBalance(value)
        }
    }
    impl ::core::convert::From<InvalidInitiator> for LiquidatorParaswapErrors {
        fn from(value: InvalidInitiator) -> Self {
            Self::InvalidInitiator(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for LiquidatorParaswapErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for LiquidatorParaswapErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for LiquidatorParaswapErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SenderNotPool> for LiquidatorParaswapErrors {
        fn from(value: SenderNotPool) -> Self {
            Self::SenderNotPool(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all input parameters for the `AUGUSTUS_REGISTRY` function with signature `AUGUSTUS_REGISTRY()` and selector `0x3a829867`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "AUGUSTUS_REGISTRY", abi = "AUGUSTUS_REGISTRY()")]
    pub struct AugustusRegistryCall;
    ///Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `0x7535d246`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `0x920f5c84`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeOperation",
        abi = "executeOperation(address[],uint256[],uint256[],address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<::ethers::core::types::U256>,
        pub initiator: ::ethers::core::types::Address,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate((address,address,uint256,bytes),address,(address,bytes))` and selector `0x903ca217`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "liquidate",
        abi = "liquidate((address,address,uint256,bytes),address,(address,bytes))"
    )]
    pub struct LiquidateCall {
        pub liquidation_params: LiquidationParams,
        pub flash_loan_pool: ::ethers::core::types::Address,
        pub swap_params: SwapParams,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LiquidatorParaswapCalls {
        AugustusRegistry(AugustusRegistryCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorParaswapCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AugustusRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AugustusRegistry(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <ExecuteOperationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteOperation(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorParaswapCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AugustusRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorParaswapCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AugustusRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AugustusRegistryCall> for LiquidatorParaswapCalls {
        fn from(value: AugustusRegistryCall) -> Self {
            Self::AugustusRegistry(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorParaswapCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for LiquidatorParaswapCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorParaswapCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorParaswapCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LiquidatorParaswapCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorParaswapCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `AUGUSTUS_REGISTRY` function with signature `AUGUSTUS_REGISTRY()` and selector `0x3a829867`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AugustusRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `POOL` function with signature `POOL()` and selector `0x7535d246`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `0x920f5c84`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExecuteOperationReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `liquidate` function with signature `liquidate((address,address,uint256,bytes),address,(address,bytes))` and selector `0x903ca217`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidateReturn {
        pub collateral_gain: ::ethers::core::types::U256,
        pub debt_gain: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
