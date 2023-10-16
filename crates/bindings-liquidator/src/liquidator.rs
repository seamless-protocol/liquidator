pub use liquidator::*;
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
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approvePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationArg1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationArg2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
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
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa\x11\x10\x80a\0a`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cj_%\x0B\x11a\0[W\x80cj_%\x0B\x14a\0\xE2W\x80c\x8D\xA5\xCB[\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W\x80c\xFAF\x1E3\x14a\x01)W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\x82W\x80cBL&[\x14a\0\xBAW\x80cW\x05\xAEC\x14a\0\xCFW[`\0\x80\xFD[a\0\x9Ds\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCDa\0\xC86`\x04a\x0C4V[a\x01<V[\0[a\0\xCDa\0\xDD6`\x04a\x0CVV[a\x01\xF9V[a\0\xF5a\0\xF06`\x04a\x0C\xC9V[a\x02\xD4V[`@Q\x90\x81R` \x01a\0\xB1V[`\0Ta\0\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xCDa\x01$6`\x04a\x0C4V[a\x04FV[a\0\xCDa\x0176`\x04a\r:V[a\x04\xBBV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`@Q\x80\x91\x03\x90\xFD[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF5\x91\x90a\r\xB3V[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02cW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02^W=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02^\x91\x90a\r\xB3V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03j\x91\x90a\r\xD5V[\x90Pa\x03\xC6\x87`@Q\x80``\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x81\x01\x8A\x90R`@\x01\x88\x90Ra\x06mV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x040\x91\x90a\r\xD5V[a\x04:\x91\x90a\x0E\x04V[\x98\x97PPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04pW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0a\x04\xC9\x82\x84\x01\x84a\x0E\x9BV[\x90P`\0\x80`\0a\x04\xDD\x84`\0\x01Qa\x07\xA1V[\x92P\x92P\x92Pa\x05\x0Bs3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x05\x06\x85\x85\x85a\x07\xE1V[a\x08LV[` \x84\x01Q\x15\x80\x15\x90a\x05!WP`@\x84\x01Q\x15\x15[\x15a\x05\xAFW` \x84\x01Q`@\x80\x86\x01Q\x90Qc\xFD!\xEC\xFF`\xE0\x1B\x81Rs\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x92c\xFD!\xEC\xFF\x92a\x05n\x92`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x9CW=`\0\x80>=`\0\xFD[PP`\0` \x87\x01\x81\x90R`@\x87\x01RPP[`\0\x80\x89\x13a\x05\xBEW\x87a\x05\xC0V[\x88[\x90Pa\x05\xCF\x85`\0\x01Qa\x08\xA1V[\x15a\x05\xEBW\x84Qa\x05\xDF\x90a\x08\xDBV[\x85Ra\x05\xEB\x81\x86a\x06mV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06a\x91\x90a\r\xB3V[PPPPPPPPPPV[`\0\x80`\0a\x06\x7F\x84`\0\x01Qa\x07\xA1V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10`\0a\x06\xBFs3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x06\xBA\x86\x88\x87a\x07\xE1V[a\t\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x06\xDB\x8Ba\x0FlV[\x86a\x07\x04Wa\x06\xFF`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0F\x88V[a\x07\x14V[a\x07\x14d\x01\0\x02v\xA3`\x01a\x0F\xA8V[\x8B`@Q` \x01a\x07%\x91\x90a\x10\x0EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07T\x95\x94\x93\x92\x91\x90a\x10JV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x96\x91\x90a\x10\x90V[PPPPPPPPPV[`\0\x80\x80a\x07\xAF\x84\x82a\t\xFBV[\x92Pa\x07\xBC\x84`\x14a\n`V[a\xFF\xFF\x16\x90Pa\x07\xD8a\x07\xD1`\x03`\x14a\x10\xB4V[\x85\x90a\t\xFBV[\x91P\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x08\x1CW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0a\x08X\x83\x83a\t\x12V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01fV[`\0a\x08\xAF`\x03`\x14a\x10\xB4V[`\x14a\x08\xBC`\x03\x82a\x10\xB4V[a\x08\xC6\x91\x90a\x10\xB4V[a\x08\xD0\x91\x90a\x10\xB4V[\x82Q\x10\x15\x90P\x91\x90PV[``a\t\x0Ca\x08\xEC`\x03`\x14a\x10\xB4V[a\x08\xF8`\x03`\x14a\x10\xB4V[\x84Qa\t\x04\x91\x90a\x10\xC7V[\x84\x91\x90a\x0B\x0BV[\x92\x91PPV[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\t:W`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0a\n\x08\x82`\x14a\x10\xB4V[\x83Q\x10\x15a\nPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x01fV[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81a\nn\x81`\x03a\x10\xB4V[\x10\x15a\n\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x01fV[a\n\xBB\x82`\x03a\x10\xB4V[\x83Q\x10\x15a\x0B\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x01fV[P\x01`\x03\x01Q\x90V[``\x81a\x0B\x19\x81`\x1Fa\x10\xB4V[\x10\x15a\x0BXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x01fV[a\x0Bb\x82\x84a\x10\xB4V[\x84Q\x10\x15a\x0B\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x01fV[``\x82\x15\x80\x15a\x0B\xC5W`@Q\x91P`\0\x82R` \x82\x01`@Ra\x0C\x0FV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x0B\xFEW\x80Q\x83R` \x92\x83\x01\x92\x01a\x0B\xE6V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C/W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0CFW`\0\x80\xFD[a\x0CO\x82a\x0C\x18V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0CiW`\0\x80\xFD[a\x0Cr\x83a\x0C\x18V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0C\x92W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xAAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0C\xC2W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x0C\xE2W`\0\x80\xFD[a\x0C\xEB\x87a\x0C\x18V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x1CW`\0\x80\xFD[a\r(\x89\x82\x8A\x01a\x0C\x80V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\rPW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\ruW`\0\x80\xFD[a\r\x81\x87\x82\x88\x01a\x0C\x80V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\r\xC5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0COW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\xE7W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0E$Wa\x0E$a\r\xEEV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EdWa\x0Eda\x0E+V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x93Wa\x0E\x93a\x0E+V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xAEW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xC6W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x0E\xDAW`\0\x80\xFD[a\x0E\xE2a\x0EAV[\x825\x82\x81\x11\x15a\x0E\xF1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x88\x13a\x0F\x02W`\0\x80\xFD[\x805\x83\x81\x11\x15a\x0F\x14Wa\x0F\x14a\x0E+V[a\x0F&`\x1F\x82\x01`\x1F\x19\x16\x87\x01a\x0EjV[\x93P\x80\x84R\x88\x86\x82\x84\x01\x01\x11\x15a\x0F<W`\0\x80\xFD[\x80\x86\x83\x01\x87\x86\x017`\0\x90\x84\x01\x86\x01RP\x90\x81R\x81\x83\x015\x92\x81\x01\x92\x90\x92R`@\x90\x81\x015\x90\x82\x01R\x93\x92PPPV[`\0`\x01`\xFF\x1B\x82\x01a\x0F\x81Wa\x0F\x81a\r\xEEV[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x0E$Wa\x0E$a\r\xEEV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x0E$Wa\x0E$a\r\xEEV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0F\xEEW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0F\xD2V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x10*`\x80\x84\x01\x82a\x0F\xC8V[\x90P` \x84\x01Q`@\x84\x01R`@\x84\x01Q``\x84\x01R\x80\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x10\x85\x90\x83\x01\x84a\x0F\xC8V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xA3W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x01\x80\x82\x11\x15a\t\x0CWa\t\x0Ca\r\xEEV[\x81\x81\x03\x81\x81\x11\x15a\t\x0CWa\t\x0Ca\r\xEEV\xFE\xA2dipfsX\"\x12 s4S\xC5Uv?9\xBF\xF6\xBE%\t\xD3'\x95\xB7$zX)RB\\=\x14\x98VU(\xBE\x0FdsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cj_%\x0B\x11a\0[W\x80cj_%\x0B\x14a\0\xE2W\x80c\x8D\xA5\xCB[\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W\x80c\xFAF\x1E3\x14a\x01)W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\x82W\x80cBL&[\x14a\0\xBAW\x80cW\x05\xAEC\x14a\0\xCFW[`\0\x80\xFD[a\0\x9Ds\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCDa\0\xC86`\x04a\x0C4V[a\x01<V[\0[a\0\xCDa\0\xDD6`\x04a\x0CVV[a\x01\xF9V[a\0\xF5a\0\xF06`\x04a\x0C\xC9V[a\x02\xD4V[`@Q\x90\x81R` \x01a\0\xB1V[`\0Ta\0\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xCDa\x01$6`\x04a\x0C4V[a\x04FV[a\0\xCDa\x0176`\x04a\r:V[a\x04\xBBV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`@Q\x80\x91\x03\x90\xFD[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF5\x91\x90a\r\xB3V[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02cW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02^W=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02^\x91\x90a\r\xB3V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03j\x91\x90a\r\xD5V[\x90Pa\x03\xC6\x87`@Q\x80``\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x81\x01\x8A\x90R`@\x01\x88\x90Ra\x06mV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x040\x91\x90a\r\xD5V[a\x04:\x91\x90a\x0E\x04V[\x98\x97PPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04pW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\r\x8DV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0a\x04\xC9\x82\x84\x01\x84a\x0E\x9BV[\x90P`\0\x80`\0a\x04\xDD\x84`\0\x01Qa\x07\xA1V[\x92P\x92P\x92Pa\x05\x0Bs3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x05\x06\x85\x85\x85a\x07\xE1V[a\x08LV[` \x84\x01Q\x15\x80\x15\x90a\x05!WP`@\x84\x01Q\x15\x15[\x15a\x05\xAFW` \x84\x01Q`@\x80\x86\x01Q\x90Qc\xFD!\xEC\xFF`\xE0\x1B\x81Rs\x8FD\xFDuB\x85\xAAj+\x8B\x9B\x97s\x9Bytn\x04u\xA7\x92c\xFD!\xEC\xFF\x92a\x05n\x92`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x9CW=`\0\x80>=`\0\xFD[PP`\0` \x87\x01\x81\x90R`@\x87\x01RPP[`\0\x80\x89\x13a\x05\xBEW\x87a\x05\xC0V[\x88[\x90Pa\x05\xCF\x85`\0\x01Qa\x08\xA1V[\x15a\x05\xEBW\x84Qa\x05\xDF\x90a\x08\xDBV[\x85Ra\x05\xEB\x81\x86a\x06mV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06a\x91\x90a\r\xB3V[PPPPPPPPPPV[`\0\x80`\0a\x06\x7F\x84`\0\x01Qa\x07\xA1V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10`\0a\x06\xBFs3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x06\xBA\x86\x88\x87a\x07\xE1V[a\t\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x06\xDB\x8Ba\x0FlV[\x86a\x07\x04Wa\x06\xFF`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0F\x88V[a\x07\x14V[a\x07\x14d\x01\0\x02v\xA3`\x01a\x0F\xA8V[\x8B`@Q` \x01a\x07%\x91\x90a\x10\x0EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07T\x95\x94\x93\x92\x91\x90a\x10JV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x96\x91\x90a\x10\x90V[PPPPPPPPPV[`\0\x80\x80a\x07\xAF\x84\x82a\t\xFBV[\x92Pa\x07\xBC\x84`\x14a\n`V[a\xFF\xFF\x16\x90Pa\x07\xD8a\x07\xD1`\x03`\x14a\x10\xB4V[\x85\x90a\t\xFBV[\x91P\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x08\x1CW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0a\x08X\x83\x83a\t\x12V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01fV[`\0a\x08\xAF`\x03`\x14a\x10\xB4V[`\x14a\x08\xBC`\x03\x82a\x10\xB4V[a\x08\xC6\x91\x90a\x10\xB4V[a\x08\xD0\x91\x90a\x10\xB4V[\x82Q\x10\x15\x90P\x91\x90PV[``a\t\x0Ca\x08\xEC`\x03`\x14a\x10\xB4V[a\x08\xF8`\x03`\x14a\x10\xB4V[\x84Qa\t\x04\x91\x90a\x10\xC7V[\x84\x91\x90a\x0B\x0BV[\x92\x91PPV[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\t:W`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0a\n\x08\x82`\x14a\x10\xB4V[\x83Q\x10\x15a\nPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x01fV[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81a\nn\x81`\x03a\x10\xB4V[\x10\x15a\n\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x01fV[a\n\xBB\x82`\x03a\x10\xB4V[\x83Q\x10\x15a\x0B\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x01fV[P\x01`\x03\x01Q\x90V[``\x81a\x0B\x19\x81`\x1Fa\x10\xB4V[\x10\x15a\x0BXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x01fV[a\x0Bb\x82\x84a\x10\xB4V[\x84Q\x10\x15a\x0B\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x01fV[``\x82\x15\x80\x15a\x0B\xC5W`@Q\x91P`\0\x82R` \x82\x01`@Ra\x0C\x0FV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x0B\xFEW\x80Q\x83R` \x92\x83\x01\x92\x01a\x0B\xE6V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C/W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0CFW`\0\x80\xFD[a\x0CO\x82a\x0C\x18V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0CiW`\0\x80\xFD[a\x0Cr\x83a\x0C\x18V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0C\x92W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xAAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0C\xC2W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x0C\xE2W`\0\x80\xFD[a\x0C\xEB\x87a\x0C\x18V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x1CW`\0\x80\xFD[a\r(\x89\x82\x8A\x01a\x0C\x80V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\rPW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\ruW`\0\x80\xFD[a\r\x81\x87\x82\x88\x01a\x0C\x80V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\r\xC5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0COW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\xE7W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0E$Wa\x0E$a\r\xEEV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EdWa\x0Eda\x0E+V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x93Wa\x0E\x93a\x0E+V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xAEW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xC6W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x0E\xDAW`\0\x80\xFD[a\x0E\xE2a\x0EAV[\x825\x82\x81\x11\x15a\x0E\xF1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x88\x13a\x0F\x02W`\0\x80\xFD[\x805\x83\x81\x11\x15a\x0F\x14Wa\x0F\x14a\x0E+V[a\x0F&`\x1F\x82\x01`\x1F\x19\x16\x87\x01a\x0EjV[\x93P\x80\x84R\x88\x86\x82\x84\x01\x01\x11\x15a\x0F<W`\0\x80\xFD[\x80\x86\x83\x01\x87\x86\x017`\0\x90\x84\x01\x86\x01RP\x90\x81R\x81\x83\x015\x92\x81\x01\x92\x90\x92R`@\x90\x81\x015\x90\x82\x01R\x93\x92PPPV[`\0`\x01`\xFF\x1B\x82\x01a\x0F\x81Wa\x0F\x81a\r\xEEV[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x0E$Wa\x0E$a\r\xEEV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x0E$Wa\x0E$a\r\xEEV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0F\xEEW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0F\xD2V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x10*`\x80\x84\x01\x82a\x0F\xC8V[\x90P` \x84\x01Q`@\x84\x01R`@\x84\x01Q``\x84\x01R\x80\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x10\x85\x90\x83\x01\x84a\x0F\xC8V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xA3W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x01\x80\x82\x11\x15a\t\x0CWa\t\x0Ca\r\xEEV[\x81\x81\x03\x81\x81\x11\x15a\t\x0CWa\t\x0Ca\r\xEEV\xFE\xA2dipfsX\"\x12 s4S\xC5Uv?9\xBF\xF6\xBE%\t\xD3'\x95\xB7$zX)RB\\=\x14\x98VU(\xBE\x0FdsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
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
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x6a5f250b) function
        pub fn liquidate(
            &self,
            collateral: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            liquidation_arg_1: [u8; 32],
            liquidation_arg_2: [u8; 32],
            swap_path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [106, 95, 37, 11],
                    (
                        collateral,
                        debt_to_cover,
                        liquidation_arg_1,
                        liquidation_arg_2,
                        swap_path,
                    ),
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
        ///Calls the contract's `pool` (0x16f0115b) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x5705ae43) function
        pub fn recover(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 5, 174, 67], (token, amount))
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
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
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
    for Liquidator<M> {
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
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approvePool` function with signature `approvePool(address)` and selector `0x424c265b`
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
    #[ethcall(name = "approvePool", abi = "approvePool(address)")]
    pub struct ApprovePoolCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,uint256,bytes32,bytes32,bytes)` and selector `0x6a5f250b`
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
        name = "liquidate",
        abi = "liquidate(address,uint256,bytes32,bytes32,bytes)"
    )]
    pub struct LiquidateCall {
        pub collateral: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub liquidation_arg_1: [u8; 32],
        pub liquidation_arg_2: [u8; 32],
        pub swap_path: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `pool` function with signature `pool()` and selector `0x16f0115b`
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
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `recover` function with signature `recover(address,uint256)` and selector `0x5705ae43`
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
    #[ethcall(name = "recover", abi = "recover(address,uint256)")]
    pub struct RecoverCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorCalls {
        ApprovePool(ApprovePoolCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        Recover(RecoverCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApprovePool(decoded));
            }
            if let Ok(decoded)
                = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded)
                = <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for LiquidatorCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,uint256,bytes32,bytes32,bytes)` and selector `0x6a5f250b`
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
    pub struct LiquidateReturn {
        pub collateral_gain: ::ethers::core::types::I256,
    }
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
    ///Container type for all return fields from the `pool` function with signature `pool()` and selector `0x16f0115b`
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
    pub struct PoolReturn(pub ::ethers::core::types::Address);
}
