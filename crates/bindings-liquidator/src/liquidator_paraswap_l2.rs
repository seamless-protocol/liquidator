pub use liquidator_paraswap_l2::*;
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
pub mod liquidator_paraswap_l2 {
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
                                        ::std::borrow::ToOwned::to_owned("contract IL2Pool"),
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
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ILiquidatorParaswapL2.LiquidationParams",
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
                                            "struct ILiquidatorParaswapL2.SwapParams",
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
    pub static LIQUIDATORPARASWAPL2_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P3\x80a\x007W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0@\x81a\0FV[Pa\0\x96V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x14\x86\x80a\0\xA5`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xDFW\x80c\x92\x0F\\\x84\x14a\0\xF0W\x80c\xC3\xA2\x80\x91\x14a\x01\x13W\x80c\xF2\xFD\xE3\x8B\x14a\x01;W`\0\x80\xFD[\x80c:\x82\x98g\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\xBAW\x80cu5\xD2F\x14a\0\xC4W[`\0\x80\xFD[a\0\x9Ds~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC2a\x01NV[\0[a\0\x9Ds\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\x9DV[a\x01\x03a\0\xFE6`\x04a\x0ERV[a\x01bV[`@Q\x90\x15\x15\x81R` \x01a\0\xB1V[a\x01&a\x01!6`\x04a\x0FWV[a\x03\x9FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB1V[a\0\xC2a\x01I6`\x04a\x0F\xC6V[a\x06\xE5V[a\x01Va\x07#V[a\x01``\0a\x07PV[V[`\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x97W`@Qc\x84qa\x8B`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x160\x14a\x01\xCBW`@Qc\x02\xA8h\xED`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x8EV[`\0a\x01\xD9\x83\x85\x01\x85a\x11'V[\x90Pa\x02F\x81`\x80\x01Q`\0\x01Q\x8C\x8C`\0\x81\x81\x10a\x01\xFAWa\x01\xFAa\x11\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x02\x0F\x91\x90a\x0F\xC6V[\x83` \x01Q\x8C\x8C`\0\x81\x81\x10a\x02'Wa\x02'a\x11\xC7V[\x90P` \x02\x015\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q` \x01Qa\x07\xA0V[`\0\x87\x87`\0\x81\x81\x10a\x02[Wa\x02[a\x11\xC7V[\x90P` \x02\x015\x8A\x8A`\0\x81\x81\x10a\x02uWa\x02ua\x11\xC7V[\x90P` \x02\x015a\x02\x86\x91\x90a\x11\xF3V[\x90P`\0\x81\x8D\x8D`\0\x81\x81\x10a\x02\x9EWa\x02\x9Ea\x11\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x02\xB3\x91\x90a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1D\x91\x90a\x12\x06V[a\x03'\x91\x90a\x12\x1FV[\x83Q\x90\x91P\x81\x10\x15a\x03OW`@Qc\x0C\x1D\xE0\xD3`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x01\x8EV[a\x03\x8C\x8D\x8D`\0\x81\x81\x10a\x03eWa\x03ea\x11\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x03z\x91\x90a\x0F\xC6V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\nJV[P`\x01\x9C\x9BPPPPPPPPPPPPV[`\0\x80a\x03\xAAa\x07#V[`\0a\x03\xBC`@\x87\x01` \x88\x01a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04&\x91\x90a\x12\x06V[\x90P`\0a\x047` \x88\x01\x88a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xA1\x91\x90a\x12\x06V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x90\x91U\x90\x91Pc\xAB\x9CK]0a\x04\xE0a\x04\xDB`@\x8C\x01` \x8D\x01a\x0F\xC6V[a\x0B\x04V[a\x04\xED\x8B`@\x015a\x0B_V[a\x04\xF7`\0a\x0B_V[0`@Q\x80`\xA0\x01`@R\x80\x8A\x81R` \x01\x8F`\0\x01` \x81\x01\x90a\x05\x1C\x91\x90a\x0F\xC6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F``\x015\x81R` \x01\x8F`\x80\x015\x81R` \x01\x8Da\x05H\x90a\x122V[\x90R`@Qa\x05Z\x91\x90` \x01a\x12\x8EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\0`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x8D\x97\x96\x95\x94\x93\x92\x91\x90a\x139V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xBBW=`\0\x80>=`\0\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP\x81\x90Pa\x05\xDE` \x89\x01\x89a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06H\x91\x90a\x12\x06V[a\x06R\x91\x90a\x12\x1FV[\x93P\x81a\x06e`@\x89\x01` \x8A\x01a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCF\x91\x90a\x12\x06V[a\x06\xD9\x91\x90a\x12\x1FV[\x92PPP\x93P\x93\x91PPV[a\x06\xEDa\x07#V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\x17W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x01\x8EV[a\x07 \x81a\x07PV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01`W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\x8EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x0B\x91\x90a\x12\x06V[\x90Pa\x08,\x87s\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x87a\nJV[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90Rs\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x80W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x94W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x92P\x83\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x05\x91\x90a\x12\x06V[a\t\x0F\x91\x90a\x12\x1FV[`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01R\x90\x91Ps~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8E\x91\x90a\x13\xF5V[a\t\xB6W`@Qc[\x15\x99\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x01a\x01\x8EV[`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x1A\x91\x90a\x14\x17V[\x90Pa\n'\x88\x82\x84a\nJV[a\n1\x8A\x85a\x0B\xA6V[Pa\n>\x88\x82`\0a\nJV[PPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\n\x9B\x84\x82a\x0B\xBDV[a\n\xFEW`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\n\xF4\x90\x85\x90a\x0CeV[a\n\xFE\x84\x82a\x0CeV[PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x0B:Wa\x0B:a\x11\xC7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x0B\x95Wa\x0B\x95a\x11\xC7V[` \x02` \x01\x01\x81\x81RPP\x91\x90PV[``a\x0B\xB4\x83\x83`\0a\x0C\xCDV[\x90P[\x92\x91PPV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0B\xDA\x91\x90a\x144V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0C\x17W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\x1CV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0CFWP\x80Q\x15\x80a\x0CFWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0CF\x91\x90a\x13\xF5V[\x80\x15a\x0C\\WP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\x0Cz`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x0B\xA6V[\x90P\x80Q`\0\x14\x15\x80\x15a\x0C\x9FWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\x9D\x91\x90a\x13\xF5V[\x15[\x15a\x0C\xC8W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x01\x8EV[PPPV[``\x81G\x10\x15a\x0C\xF2W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x01\x8EV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\r\x0E\x91\x90a\x144V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\rKW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\rPV[``\x91P[P\x91P\x91Pa\r`\x86\x83\x83a\rlV[\x92PPP[\x93\x92PPPV[``\x82a\r\x81Wa\r|\x82a\r\xC8V[a\reV[\x81Q\x15\x80\x15a\r\x98WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\r\xC1W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x8EV[P\x80a\reV[\x80Q\x15a\r\xD8W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x0E\x03W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x1BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0E6W`\0\x80\xFD[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07 W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\x0EpW`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x88W`\0\x80\xFD[a\x0E\x94\x8D\x83\x8E\x01a\r\xF1V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x0E\xADW`\0\x80\xFD[a\x0E\xB9\x8D\x83\x8E\x01a\r\xF1V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x0E\xD2W`\0\x80\xFD[a\x0E\xDE\x8D\x83\x8E\x01a\r\xF1V[\x90\x97P\x95P``\x8C\x015\x91Pa\x0E\xF3\x82a\x0E=V[\x90\x93P`\x80\x8B\x015\x90\x80\x82\x11\x15a\x0F\tW`\0\x80\xFD[\x81\x8C\x01\x91P\x8C`\x1F\x83\x01\x12a\x0F\x1DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F,W`\0\x80\xFD[\x8D` \x82\x85\x01\x01\x11\x15a\x0F>W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a\x0FmW`\0\x80\xFD[`\xA0\x81\x12\x15a\x0F{W`\0\x80\xFD[P\x83\x92P`\xA0\x84\x015a\x0F\x8D\x81a\x0E=V[\x91P`\xC0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xA9W`\0\x80\xFD[\x84\x01`@\x81\x87\x03\x12\x15a\x0F\xBBW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD8W`\0\x80\xFD[\x815a\re\x81a\x0E=V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x1CWa\x10\x1Ca\x0F\xE3V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x1CWa\x10\x1Ca\x0F\xE3V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10nWa\x10na\x0F\xE3V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x10\x88W`\0\x80\xFD[a\x10\x90a\x0F\xF9V[\x90P\x815a\x10\x9D\x81a\x0E=V[\x81R` \x82\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xBBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10\xCFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10\xE1Wa\x10\xE1a\x0F\xE3V[a\x10\xF3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x10EV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x11\tW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x84\x01R\x91\x83\x01\x91\x90\x91RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x119W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11QW`\0\x80\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a\x11eW`\0\x80\xFD[a\x11ma\x10\"V[\x825\x81R` \x83\x015a\x11\x7F\x81a\x0E=V[\x80` \x83\x01RP`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x11\xACW`\0\x80\xFD[a\x11\xB8\x87\x82\x86\x01a\x10vV[`\x80\x83\x01RP\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xB7Wa\x0B\xB7a\x11\xDDV[`\0` \x82\x84\x03\x12\x15a\x12\x18W`\0\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xB7Wa\x0B\xB7a\x11\xDDV[`\0a\x0B\xB76\x83a\x10vV[`\0[\x83\x81\x10\x15a\x12YW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12AV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12z\x81` \x86\x01` \x86\x01a\x12>V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16`@\x85\x01R`@\x85\x01Q``\x85\x01R``\x85\x01Q`\x80\x85\x01R`\x80\x85\x01Q\x91P`\xA0\x80\x85\x01R\x80\x82Q\x16`\xC0\x85\x01RP` \x81\x01Q\x90P`@`\xE0\x84\x01Ra\x12\xF6a\x01\0\x84\x01\x82a\x12bV[\x94\x93PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x13.W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x13\x12V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x13\x88W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x13jV[PP\x85\x81\x03`@\x87\x01Ra\x13\x9C\x81\x8Ca\x12\xFEV[\x93PPPP\x82\x81\x03``\x84\x01Ra\x13\xB3\x81\x88a\x12\xFEV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x13\xD6\x81\x86a\x12bV[\x91PPa\x13\xE9`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x14\x07W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\reW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x14)W`\0\x80\xFD[\x81Qa\re\x81a\x0E=V[`\0\x82Qa\x14F\x81\x84` \x87\x01a\x12>V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xBF7\r\xAD\xA4\x83J\x8A\xF8yl\xD8sUN\xA04\x1E\x9F\x0B\x06\x03\x1D\x198\x047RV\xC2\xA2WdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATORPARASWAPL2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xDFW\x80c\x92\x0F\\\x84\x14a\0\xF0W\x80c\xC3\xA2\x80\x91\x14a\x01\x13W\x80c\xF2\xFD\xE3\x8B\x14a\x01;W`\0\x80\xFD[\x80c:\x82\x98g\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\xBAW\x80cu5\xD2F\x14a\0\xC4W[`\0\x80\xFD[a\0\x9Ds~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC2a\x01NV[\0[a\0\x9Ds\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\x9DV[a\x01\x03a\0\xFE6`\x04a\x0ERV[a\x01bV[`@Q\x90\x15\x15\x81R` \x01a\0\xB1V[a\x01&a\x01!6`\x04a\x0FWV[a\x03\x9FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB1V[a\0\xC2a\x01I6`\x04a\x0F\xC6V[a\x06\xE5V[a\x01Va\x07#V[a\x01``\0a\x07PV[V[`\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x97W`@Qc\x84qa\x8B`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x160\x14a\x01\xCBW`@Qc\x02\xA8h\xED`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x8EV[`\0a\x01\xD9\x83\x85\x01\x85a\x11'V[\x90Pa\x02F\x81`\x80\x01Q`\0\x01Q\x8C\x8C`\0\x81\x81\x10a\x01\xFAWa\x01\xFAa\x11\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x02\x0F\x91\x90a\x0F\xC6V[\x83` \x01Q\x8C\x8C`\0\x81\x81\x10a\x02'Wa\x02'a\x11\xC7V[\x90P` \x02\x015\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q` \x01Qa\x07\xA0V[`\0\x87\x87`\0\x81\x81\x10a\x02[Wa\x02[a\x11\xC7V[\x90P` \x02\x015\x8A\x8A`\0\x81\x81\x10a\x02uWa\x02ua\x11\xC7V[\x90P` \x02\x015a\x02\x86\x91\x90a\x11\xF3V[\x90P`\0\x81\x8D\x8D`\0\x81\x81\x10a\x02\x9EWa\x02\x9Ea\x11\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x02\xB3\x91\x90a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1D\x91\x90a\x12\x06V[a\x03'\x91\x90a\x12\x1FV[\x83Q\x90\x91P\x81\x10\x15a\x03OW`@Qc\x0C\x1D\xE0\xD3`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x01\x8EV[a\x03\x8C\x8D\x8D`\0\x81\x81\x10a\x03eWa\x03ea\x11\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x03z\x91\x90a\x0F\xC6V[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\nJV[P`\x01\x9C\x9BPPPPPPPPPPPPV[`\0\x80a\x03\xAAa\x07#V[`\0a\x03\xBC`@\x87\x01` \x88\x01a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04&\x91\x90a\x12\x06V[\x90P`\0a\x047` \x88\x01\x88a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xA1\x91\x90a\x12\x06V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x90\x91U\x90\x91Pc\xAB\x9CK]0a\x04\xE0a\x04\xDB`@\x8C\x01` \x8D\x01a\x0F\xC6V[a\x0B\x04V[a\x04\xED\x8B`@\x015a\x0B_V[a\x04\xF7`\0a\x0B_V[0`@Q\x80`\xA0\x01`@R\x80\x8A\x81R` \x01\x8F`\0\x01` \x81\x01\x90a\x05\x1C\x91\x90a\x0F\xC6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F``\x015\x81R` \x01\x8F`\x80\x015\x81R` \x01\x8Da\x05H\x90a\x122V[\x90R`@Qa\x05Z\x91\x90` \x01a\x12\x8EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\0`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x8D\x97\x96\x95\x94\x93\x92\x91\x90a\x139V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xBBW=`\0\x80>=`\0\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP\x81\x90Pa\x05\xDE` \x89\x01\x89a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06H\x91\x90a\x12\x06V[a\x06R\x91\x90a\x12\x1FV[\x93P\x81a\x06e`@\x89\x01` \x8A\x01a\x0F\xC6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCF\x91\x90a\x12\x06V[a\x06\xD9\x91\x90a\x12\x1FV[\x92PPP\x93P\x93\x91PPV[a\x06\xEDa\x07#V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\x17W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x01\x8EV[a\x07 \x81a\x07PV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01`W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\x8EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x0B\x91\x90a\x12\x06V[\x90Pa\x08,\x87s\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x87a\nJV[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90Rs\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x80W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x94W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x92P\x83\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x05\x91\x90a\x12\x06V[a\t\x0F\x91\x90a\x12\x1FV[`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01R\x90\x91Ps~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8E\x91\x90a\x13\xF5V[a\t\xB6W`@Qc[\x15\x99\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x01a\x01\x8EV[`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x1A\x91\x90a\x14\x17V[\x90Pa\n'\x88\x82\x84a\nJV[a\n1\x8A\x85a\x0B\xA6V[Pa\n>\x88\x82`\0a\nJV[PPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\n\x9B\x84\x82a\x0B\xBDV[a\n\xFEW`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\n\xF4\x90\x85\x90a\x0CeV[a\n\xFE\x84\x82a\x0CeV[PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x0B:Wa\x0B:a\x11\xC7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x0B\x95Wa\x0B\x95a\x11\xC7V[` \x02` \x01\x01\x81\x81RPP\x91\x90PV[``a\x0B\xB4\x83\x83`\0a\x0C\xCDV[\x90P[\x92\x91PPV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0B\xDA\x91\x90a\x144V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0C\x17W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\x1CV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0CFWP\x80Q\x15\x80a\x0CFWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0CF\x91\x90a\x13\xF5V[\x80\x15a\x0C\\WP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\x0Cz`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x0B\xA6V[\x90P\x80Q`\0\x14\x15\x80\x15a\x0C\x9FWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\x9D\x91\x90a\x13\xF5V[\x15[\x15a\x0C\xC8W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x01\x8EV[PPPV[``\x81G\x10\x15a\x0C\xF2W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x01\x8EV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\r\x0E\x91\x90a\x144V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\rKW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\rPV[``\x91P[P\x91P\x91Pa\r`\x86\x83\x83a\rlV[\x92PPP[\x93\x92PPPV[``\x82a\r\x81Wa\r|\x82a\r\xC8V[a\reV[\x81Q\x15\x80\x15a\r\x98WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\r\xC1W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x8EV[P\x80a\reV[\x80Q\x15a\r\xD8W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x0E\x03W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x1BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0E6W`\0\x80\xFD[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07 W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\x0EpW`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x88W`\0\x80\xFD[a\x0E\x94\x8D\x83\x8E\x01a\r\xF1V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x0E\xADW`\0\x80\xFD[a\x0E\xB9\x8D\x83\x8E\x01a\r\xF1V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x0E\xD2W`\0\x80\xFD[a\x0E\xDE\x8D\x83\x8E\x01a\r\xF1V[\x90\x97P\x95P``\x8C\x015\x91Pa\x0E\xF3\x82a\x0E=V[\x90\x93P`\x80\x8B\x015\x90\x80\x82\x11\x15a\x0F\tW`\0\x80\xFD[\x81\x8C\x01\x91P\x8C`\x1F\x83\x01\x12a\x0F\x1DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F,W`\0\x80\xFD[\x8D` \x82\x85\x01\x01\x11\x15a\x0F>W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a\x0FmW`\0\x80\xFD[`\xA0\x81\x12\x15a\x0F{W`\0\x80\xFD[P\x83\x92P`\xA0\x84\x015a\x0F\x8D\x81a\x0E=V[\x91P`\xC0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xA9W`\0\x80\xFD[\x84\x01`@\x81\x87\x03\x12\x15a\x0F\xBBW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD8W`\0\x80\xFD[\x815a\re\x81a\x0E=V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x1CWa\x10\x1Ca\x0F\xE3V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x1CWa\x10\x1Ca\x0F\xE3V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10nWa\x10na\x0F\xE3V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x10\x88W`\0\x80\xFD[a\x10\x90a\x0F\xF9V[\x90P\x815a\x10\x9D\x81a\x0E=V[\x81R` \x82\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xBBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10\xCFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10\xE1Wa\x10\xE1a\x0F\xE3V[a\x10\xF3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x10EV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x11\tW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x84\x01R\x91\x83\x01\x91\x90\x91RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x119W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11QW`\0\x80\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a\x11eW`\0\x80\xFD[a\x11ma\x10\"V[\x825\x81R` \x83\x015a\x11\x7F\x81a\x0E=V[\x80` \x83\x01RP`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x11\xACW`\0\x80\xFD[a\x11\xB8\x87\x82\x86\x01a\x10vV[`\x80\x83\x01RP\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xB7Wa\x0B\xB7a\x11\xDDV[`\0` \x82\x84\x03\x12\x15a\x12\x18W`\0\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xB7Wa\x0B\xB7a\x11\xDDV[`\0a\x0B\xB76\x83a\x10vV[`\0[\x83\x81\x10\x15a\x12YW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12AV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12z\x81` \x86\x01` \x86\x01a\x12>V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16`@\x85\x01R`@\x85\x01Q``\x85\x01R``\x85\x01Q`\x80\x85\x01R`\x80\x85\x01Q\x91P`\xA0\x80\x85\x01R\x80\x82Q\x16`\xC0\x85\x01RP` \x81\x01Q\x90P`@`\xE0\x84\x01Ra\x12\xF6a\x01\0\x84\x01\x82a\x12bV[\x94\x93PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x13.W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x13\x12V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x13\x88W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x13jV[PP\x85\x81\x03`@\x87\x01Ra\x13\x9C\x81\x8Ca\x12\xFEV[\x93PPPP\x82\x81\x03``\x84\x01Ra\x13\xB3\x81\x88a\x12\xFEV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x13\xD6\x81\x86a\x12bV[\x91PPa\x13\xE9`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x14\x07W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\reW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x14)W`\0\x80\xFD[\x81Qa\re\x81a\x0E=V[`\0\x82Qa\x14F\x81\x84` \x87\x01a\x12>V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xBF7\r\xAD\xA4\x83J\x8A\xF8yl\xD8sUN\xA04\x1E\x9F\x0B\x06\x03\x1D\x198\x047RV\xC2\xA2WdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATORPARASWAPL2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LiquidatorParaswapL2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LiquidatorParaswapL2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LiquidatorParaswapL2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LiquidatorParaswapL2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LiquidatorParaswapL2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LiquidatorParaswapL2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LiquidatorParaswapL2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATORPARASWAPL2_ABI.clone(),
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
                LIQUIDATORPARASWAPL2_ABI.clone(),
                LIQUIDATORPARASWAPL2_BYTECODE.clone().into(),
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
        ///Calls the contract's `liquidate` (0xc3a28091) function
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
                    [195, 162, 128, 145],
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
    for LiquidatorParaswapL2<M> {
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
    pub enum LiquidatorParaswapL2Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
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
    impl ::ethers::core::abi::AbiDecode for LiquidatorParaswapL2Errors {
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
    impl ::ethers::core::abi::AbiEncode for LiquidatorParaswapL2Errors {
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
    impl ::ethers::contract::ContractRevert for LiquidatorParaswapL2Errors {
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
    impl ::core::fmt::Display for LiquidatorParaswapL2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<::std::string::String> for LiquidatorParaswapL2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LiquidatorParaswapL2Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for LiquidatorParaswapL2Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LiquidatorParaswapL2Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InvalidAugustusInstance> for LiquidatorParaswapL2Errors {
        fn from(value: InvalidAugustusInstance) -> Self {
            Self::InvalidAugustusInstance(value)
        }
    }
    impl ::core::convert::From<InvalidDebtBalance> for LiquidatorParaswapL2Errors {
        fn from(value: InvalidDebtBalance) -> Self {
            Self::InvalidDebtBalance(value)
        }
    }
    impl ::core::convert::From<InvalidInitiator> for LiquidatorParaswapL2Errors {
        fn from(value: InvalidInitiator) -> Self {
            Self::InvalidInitiator(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for LiquidatorParaswapL2Errors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount>
    for LiquidatorParaswapL2Errors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for LiquidatorParaswapL2Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SenderNotPool> for LiquidatorParaswapL2Errors {
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
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate((address,address,uint256,bytes32,bytes32),address,(address,bytes))` and selector `0xc3a28091`
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
        abi = "liquidate((address,address,uint256,bytes32,bytes32),address,(address,bytes))"
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
    pub enum LiquidatorParaswapL2Calls {
        AugustusRegistry(AugustusRegistryCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorParaswapL2Calls {
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
    impl ::ethers::core::abi::AbiEncode for LiquidatorParaswapL2Calls {
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
    impl ::core::fmt::Display for LiquidatorParaswapL2Calls {
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
    impl ::core::convert::From<AugustusRegistryCall> for LiquidatorParaswapL2Calls {
        fn from(value: AugustusRegistryCall) -> Self {
            Self::AugustusRegistry(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorParaswapL2Calls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for LiquidatorParaswapL2Calls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorParaswapL2Calls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorParaswapL2Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LiquidatorParaswapL2Calls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorParaswapL2Calls {
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
    ///Container type for all return fields from the `liquidate` function with signature `liquidate((address,address,uint256,bytes32,bytes32),address,(address,bytes))` and selector `0xc3a28091`
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
