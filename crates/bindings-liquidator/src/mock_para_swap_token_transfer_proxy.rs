pub use mock_para_swap_token_transfer_proxy::*;
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
pub mod mock_para_swap_token_transfer_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedContracts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failed"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mockSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
                    ::std::borrow::ToOwned::to_owned("setToAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setToAmount"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "targetArtifactSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifactSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
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
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedContracts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
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
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("SlotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SlotFound"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("who"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fsig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("keysHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WARNING_UninitedSlot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WARNING_UninitedSlot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("who"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_int",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_uint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("logs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
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
    pub static MOCKPARASWAPTOKENTRANSFERPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa#x\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0qW\x80c\x91j\x17\xC6\x14a\x01&W\x80c\xB1\x14\x10\xAF\x14a\x01.W\x80c\xB5P\x8A\xA9\x14a\x01AW\x80c\xBAAO\xA6\x14a\x01IW\x80c\xE2\x0C\x9Fq\x14a\x01aW\x80c\xFAv&\xD4\x14a\x01iW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xB9W\x80c>^<#\x14a\0\xD7W\x80c?r\x86\xF4\x14a\0\xDFW\x80c`\x15M\xA9\x14a\0\xE7W\x80cf\xD9\xA9\xA0\x14a\0\xFCW\x80c\x85\"l\x81\x14a\x01\x11W[`\0\x80\xFD[a\0\xC1a\x01vV[`@Qa\0\xCE\x91\x90a\x1D\x92V[`@Q\x80\x91\x03\x90\xF3[a\0\xC1a\x01\xD8V[a\0\xC1a\x028V[a\0\xFAa\0\xF56`\x04a\x1D\xF6V[a\x02\x98V[\0[a\x01\x04a\x03\x97V[`@Qa\0\xCE\x91\x90a\x1E2V[a\x01\x19a\x04\x86V[`@Qa\0\xCE\x91\x90a\x1F\tV[a\x01\x04a\x05VV[a\0\xFAa\x01<6`\x04a\x1F\x83V[`\x1BUV[a\x01\x19a\x06<V[a\x01Qa\x07\x0CV[`@Q\x90\x15\x15\x81R` \x01a\0\xCEV[a\0\xC1a\x087V[`\0Ta\x01Q\x90`\xFF\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0WPPPPP\x90P\x90V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x0F\x91\x90a\x1F\x9CV[P`\x1BT`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01\x81\x90Ra\x03\x92\x92\x85\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x83\x91\x90a\x1F\xC5V[a\x03\x8D\x91\x90a\x1F\xF4V[a\x08\x97V[PPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x04eW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x04'W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x03\xBBV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04\xC9\x90a \x07V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xF5\x90a \x07V[\x80\x15a\x05BW\x80`\x1F\x10a\x05\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\xAAV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x06$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x05\xE6W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05zV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\x7F\x90a \x07V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xAB\x90a \x07V[\x80\x15a\x06\xF8W\x80`\x1F\x10a\x06\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xF8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06`V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x07,WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x082W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x07\xBA\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xD4\x91a rV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08\x11W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x16V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x08.\x91\x90a\x1F\x9CV[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0WPPPPP\x90P\x90V[a\x03\x92\x83\x83\x83`\0`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`\0\x91\x90\x86\x16\x90cp\xA0\x821\x90`D\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x08\xFB\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\t6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t;V[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\tU\x91\x90a\x1F\xC5V[\x90Pa\t\x87\x84a\t\x81\x87a\t{cp\xA0\x821`\xE0\x1Ba\tu`\x05\x8Da\n\x8AV[\x90a\n\xB4V[\x90a\n\xD1V[\x90a\n\xF9V[\x82\x15a\n\x82W`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x18\x16\r\xDD`\xE0\x1B\x17\x90R\x90Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91a\t\xCF\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\n\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x0FV[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n)\x91\x90a\x1F\xC5V[\x90P\x82\x86\x10\x15a\nNWa\n=\x86\x84a \x8EV[a\nG\x90\x82a \x8EV[\x90Pa\neV[a\nX\x83\x87a \x8EV[a\nb\x90\x82a\x1F\xF4V[\x90P[a\n\x7F\x81a\t\x81c\x18\x16\r\xDD`\xE0\x1Ba\tu`\x05\x8Da\n\x8AV[PP[PPPPPPV[`\x05\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\0\x82[\x90P[\x92\x91PPV[`\x03\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16`\xE0\x83\x90\x1C\x17\x90U`\0\x82a\n\xABV[`\x02\x82\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x82 `\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x01U\x82a\n\xABV[a\x0B\x03\x82\x82a\x0B\x07V[PPV[`\x05\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x02\x85\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x95`\xE0\x95\x90\x95\x1B\x94`\0\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x0BxW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0BdW[PPPPP\x90P`\0\x83a\x0B\x8B\x83a\x0EgV[`@Q` \x01a\x0B\x9C\x92\x91\x90a AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01\x8B\x01` \x90\x81R\x83\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x83R\x81R\x92\x81 \x91\x94P\x90\x92\x90\x91a\x0B\xEE\x91\x86\x91\x88\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x0C&Wa\x0C$\x87a\x0F\x0EV[P[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x88\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x88\x16\x84R\x82R\x80\x83 \x90Q\x90\x91\x83\x91a\x0Ce\x91\x87\x91\x89\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T`\0\x1B\x90P`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0C\xAA\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x0C\xE5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xEAV[``\x91P[P\x91Pa\r\x03\x90P\x81a\x0C\xFE\x88` a \xDBV[a\x0F\x19V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90a\x1F\xC5V[\x90P\x80\x82\x14a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xAE\x90a \xF2V[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xCA\x10\xBB`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cp\xCA\x10\xBB\x90a\r\xF2\x90\x8B\x90\x87\x90\x8E\x90`\x04\x01a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x0CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E W=`\0\x80>=`\0\xFD[PPP`\x05\x8B\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP`\x03\x8A\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x0ES`\x02\x8B\x01`\0a\x1DXV[\x89`\x04\x01`\0\x90UPPPPPPPPPPV[```\0\x82Q` a\x0Ey\x91\x90a \xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x91Wa\x0E\x91a!\xAEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\xBBW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\x07W`\0\x84\x82\x81Q\x81\x10a\x0E\xDEWa\x0E\xDEa!\xC4V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x0E\xFF\x90a!\xDAV[\x91PPa\x0E\xC1V[P\x92\x91PPV[`\0a\n\xAE\x82a\x0F\x96V[`\0\x80`\0` \x85Q\x11a\x0F.W\x84Qa\x0F1V[` [\x90P`\0[\x81\x81\x10\x15a\x0F\x8CWa\x0FI\x81`\x08a \xDBV[\x86a\x0FT\x83\x88a\x1F\xF4V[\x81Q\x81\x10a\x0FdWa\x0Fda!\xC4V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x0F\x84\x81a!\xDAV[\x91PPa\x0F6V[P\x90\x94\x93PPPPV[`\x05\x81\x01T`\x03\x82\x01T`\x04\x83\x01T`\x02\x84\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\0\x96`\x01`\x01`\xA0\x1B\x03\x16\x95`\xE0\x1B\x94\x93\x87\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x10\x06W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0F\xF2W[PPP`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x95\x96P\x94\x91\x93Pa\x10P\x92P\x85\x91\x87\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16\x15a\x10\xECW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x10\xBC\x91\x85\x91\x87\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x94PPPPP\x91\x90PV[`\0\x83a\x10\xF8\x83a\x1CEV[`@Q` \x01a\x11\t\x92\x91\x90a AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c&l\xF1\t`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11fW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11zW=`\0\x80>=`\0\xFD[PPPP`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x11\x99\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x11\xD4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\xD9V[``\x91P[P\x91Pa\x11\xF2\x90P\x81a\x11\xED\x87` a \xDBV[a\x1C\xE5V[`@Qce\xBC\x94\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\x04\x82\x01R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ce\xBC\x94\x81\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12{\x91\x90\x81\x01\x90a\"\x8EV[P\x90P\x80Q`\x01\x03a\x15=W`\0`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x89\x84`\0\x81Q\x81\x10a\x12\xBDWa\x12\xBDa!\xC4V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xF6\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x137\x91\x90a\x1F\xC5V[\x90P\x80a\x13\x9EW\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x88\x83`\0\x81Q\x81\x10a\x13sWa\x13sa!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[\x80\x83\x14a\x13\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xAE\x90a \xF2V[\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x88\x88\x87\x89`@Q` \x01a\x13\xF3\x92\x91\x90a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`\0\x81Q\x81\x10a\x14\x1CWa\x14\x1Ca!\xC4V[` \x02` \x01\x01Q`\0\x1C`@Qa\x147\x94\x93\x92\x91\x90a\"\xF2V[`@Q\x80\x91\x03\x90\xA1\x81`\0\x81Q\x81\x10a\x14RWa\x14Ra!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R\x8C\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x14\x9B\x91\x8A\x91\x8C\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x82\x82\x01\x93\x90\x93R\x90\x82\x01`\0\x90\x81 \x93\x90\x93U`\x01`\x01`\xA0\x1B\x03\x8B\x16\x83R`\x01\x8D\x81\x01\x82R\x82\x84 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x85R\x82R\x82\x84 \x92Q\x90\x93\x91a\x15\x03\x91\x8A\x91\x8C\x91\x01a \xA1V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPa\x1A\xD0V[`\x01\x81Q\x11\x15a\x1A`W`\0[\x81Q\x81\x10\x15a\x1AZW`\0`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x8A\x85\x85\x81Q\x81\x10a\x15\x88Wa\x15\x88a!\xC4V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xC1\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x02\x91\x90a\x1F\xC5V[\x90P\x80a\x16hW\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x89\x84\x84\x81Q\x81\x10a\x16=Wa\x16=a!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8A\x85\x85\x81Q\x81\x10a\x16\x9BWa\x16\x9Ba!\xC4V[` \x02` \x01\x01Qa\x137`\xF0\x1B`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xC7\x93\x92\x91\x90a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xF5W=`\0\x80>=`\0\xFD[PPPP`\0``\x8A`\x01`\x01`\xA0\x1B\x03\x16\x87`@Qa\x17\x15\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x17PW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x17UV[``\x91P[P\x90\x92P\x90Pa\x17j\x81a\x11\xED\x8B` a \xDBV[\x95P\x81\x80\x15a\x17}WPa\x137`\xF0\x1B\x86\x14[\x15a\x19\xB8W\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x8B\x8B\x8A\x8C`@Q` \x01a\x17\xB8\x92\x91\x90a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88\x81Q\x81\x10a\x17\xE0Wa\x17\xE0a!\xC4V[` \x02` \x01\x01Q`\0\x1C`@Qa\x17\xFB\x94\x93\x92\x91\x90a\"\xF2V[`@Q\x80\x91\x03\x90\xA1\x84\x84\x81Q\x81\x10a\x18\x15Wa\x18\x15a!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R\x8F\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8F\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x18^\x91\x8D\x91\x8F\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x8D`\x01\x01`\0\x8D`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C`\x01`\x01`\xE0\x1B\x03\x19\x16`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x8C`@Q` \x01a\x18\xE9\x92\x91\x90a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x19WWa\x19Wa!\xC4V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19~\x93\x92\x91\x90a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xACW=`\0\x80>=`\0\xFD[PPPPPPPa\x1AZV[`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x19\xEBWa\x19\xEBa!\xC4V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x12\x93\x92\x91\x90a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A@W=`\0\x80>=`\0\xFD[PPPPPPP\x80\x80a\x1AR\x90a!\xDAV[\x91PPa\x15JV[Pa\x1A\xD0V[`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FstdStorage find(StdStorage): No `D\x82\x01R\x7Fstorage use detected for target.`d\x82\x01R`\x84\x01a\r\xAEV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1B\x12\x91\x88\x91\x8A\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x1B\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FstdStorage find(StdStorage): Slo`D\x82\x01Rn:\x149\x94\x9077\xBA\x1037\xBA\xB72\x17`\x89\x1B`d\x82\x01R`\x84\x01a\r\xAEV[`\x05\x89\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\x03\x89\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x1B\xCE`\x02\x8A\x01`\0a\x1DXV[`\0`\x04\x8A\x01\x81\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R` \x8A\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1C\x12\x91\x88\x91\x8A\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x97PPPPPPPP\x91\x90PV[```\0\x82Q` a\x1CW\x91\x90a \xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1CoWa\x1Coa!\xAEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1C\x99W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\x07W`\0\x84\x82\x81Q\x81\x10a\x1C\xBCWa\x1C\xBCa!\xC4V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x1C\xDD\x90a!\xDAV[\x91PPa\x1C\x9FV[`\0\x80`\0` \x85Q\x11a\x1C\xFAW\x84Qa\x1C\xFDV[` [\x90P`\0[\x81\x81\x10\x15a\x0F\x8CWa\x1D\x15\x81`\x08a \xDBV[\x86a\x1D \x83\x88a\x1F\xF4V[\x81Q\x81\x10a\x1D0Wa\x1D0a!\xC4V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x1DP\x81a!\xDAV[\x91PPa\x1D\x02V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x1Dv\x91\x90a\x1DyV[PV[[\x80\x82\x11\x15a\x1D\x8EW`\0\x81U`\x01\x01a\x1DzV[P\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1D\xD3W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1D\xAEV[P\x90\x96\x95PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x082W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\x0BW`\0\x80\xFD[a\x1E\x14\x84a\x1D\xDFV[\x92Pa\x1E\"` \x85\x01a\x1D\xDFV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x1E\xD6W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x1E\xC1W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x1E\x97V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x1EZV[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x1F\0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1E\xE8V[PP`\0\x91\x01RV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1FvW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra\x1FW\x81\x89\x89\x01\x8A\x85\x01a\x1E\xE5V[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1F0V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\x95W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\xAEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\xBEW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xD7W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n\xAEWa\n\xAEa\x1F\xDEV[`\x01\x81\x81\x1C\x90\x82\x16\x80a \x1BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a ;WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a d\x81`\x04\x85\x01` \x87\x01a\x1E\xE5V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa \x84\x81\x84` \x87\x01a\x1E\xE5V[\x91\x90\x91\x01\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\n\xAEWa\n\xAEa\x1F\xDEV[\x82Q`\0\x90\x82\x90` \x80\x87\x01\x84[\x83\x81\x10\x15a \xCBW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a \xAFV[PP\x94\x82RP\x90\x92\x01\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xAEWa\n\xAEa\x1F\xDEV[` \x80\x82R`o\x90\x82\x01R\x7FstdStorage find(StdStorage): Pac`@\x82\x01R\x7Fked slot. This would cause dange``\x82\x01R\x7Frous overwriting and currently i`\x80\x82\x01Rn9\xB7\x13\xBA\x109\xBA\xB887\xB9:2\xB2\x17`\x89\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a!\xECWa!\xECa\x1F\xDEV[P`\x01\x01\x90V[`\0\x82`\x1F\x83\x01\x12a\"\x04W`\0\x80\xFD[\x81Q` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x11\x15a\"!Wa\"!a!\xAEV[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15a\"FWa\"Fa!\xAEV[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15a\"dW`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15a\"\x83W\x81Q\x83R\x91\x83\x01\x91\x90\x83\x01\x90a\"jV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xA1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xB9W`\0\x80\xFD[a\"\xC5\x86\x83\x87\x01a!\xF3V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\"\xDBW`\0\x80\xFD[Pa\"\xE8\x85\x82\x86\x01a!\xF3V[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V\xFE\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \x91u\xC5\x9D/\xEC\xE1S\xC8\xFD\xE6\x8F\xC5\x8B\xB4p\t\x01=\x973+\xD7\xEC\x84S\x87\x8Dw\xE4&\x05dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MOCKPARASWAPTOKENTRANSFERPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0qW\x80c\x91j\x17\xC6\x14a\x01&W\x80c\xB1\x14\x10\xAF\x14a\x01.W\x80c\xB5P\x8A\xA9\x14a\x01AW\x80c\xBAAO\xA6\x14a\x01IW\x80c\xE2\x0C\x9Fq\x14a\x01aW\x80c\xFAv&\xD4\x14a\x01iW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xB9W\x80c>^<#\x14a\0\xD7W\x80c?r\x86\xF4\x14a\0\xDFW\x80c`\x15M\xA9\x14a\0\xE7W\x80cf\xD9\xA9\xA0\x14a\0\xFCW\x80c\x85\"l\x81\x14a\x01\x11W[`\0\x80\xFD[a\0\xC1a\x01vV[`@Qa\0\xCE\x91\x90a\x1D\x92V[`@Q\x80\x91\x03\x90\xF3[a\0\xC1a\x01\xD8V[a\0\xC1a\x028V[a\0\xFAa\0\xF56`\x04a\x1D\xF6V[a\x02\x98V[\0[a\x01\x04a\x03\x97V[`@Qa\0\xCE\x91\x90a\x1E2V[a\x01\x19a\x04\x86V[`@Qa\0\xCE\x91\x90a\x1F\tV[a\x01\x04a\x05VV[a\0\xFAa\x01<6`\x04a\x1F\x83V[`\x1BUV[a\x01\x19a\x06<V[a\x01Qa\x07\x0CV[`@Q\x90\x15\x15\x81R` \x01a\0\xCEV[a\0\xC1a\x087V[`\0Ta\x01Q\x90`\xFF\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0WPPPPP\x90P\x90V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x0F\x91\x90a\x1F\x9CV[P`\x1BT`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01\x81\x90Ra\x03\x92\x92\x85\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x83\x91\x90a\x1F\xC5V[a\x03\x8D\x91\x90a\x1F\xF4V[a\x08\x97V[PPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x04eW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x04'W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x03\xBBV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04\xC9\x90a \x07V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xF5\x90a \x07V[\x80\x15a\x05BW\x80`\x1F\x10a\x05\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\xAAV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x06$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x05\xE6W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05zV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x04}W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\x7F\x90a \x07V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xAB\x90a \x07V[\x80\x15a\x06\xF8W\x80`\x1F\x10a\x06\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xF8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06`V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x07,WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x082W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x07\xBA\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xD4\x91a rV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08\x11W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x16V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x08.\x91\x90a\x1F\x9CV[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xCEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xB0WPPPPP\x90P\x90V[a\x03\x92\x83\x83\x83`\0`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`\0\x91\x90\x86\x16\x90cp\xA0\x821\x90`D\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x08\xFB\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\t6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t;V[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\tU\x91\x90a\x1F\xC5V[\x90Pa\t\x87\x84a\t\x81\x87a\t{cp\xA0\x821`\xE0\x1Ba\tu`\x05\x8Da\n\x8AV[\x90a\n\xB4V[\x90a\n\xD1V[\x90a\n\xF9V[\x82\x15a\n\x82W`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x18\x16\r\xDD`\xE0\x1B\x17\x90R\x90Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91a\t\xCF\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\n\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x0FV[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n)\x91\x90a\x1F\xC5V[\x90P\x82\x86\x10\x15a\nNWa\n=\x86\x84a \x8EV[a\nG\x90\x82a \x8EV[\x90Pa\neV[a\nX\x83\x87a \x8EV[a\nb\x90\x82a\x1F\xF4V[\x90P[a\n\x7F\x81a\t\x81c\x18\x16\r\xDD`\xE0\x1Ba\tu`\x05\x8Da\n\x8AV[PP[PPPPPPV[`\x05\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\0\x82[\x90P[\x92\x91PPV[`\x03\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16`\xE0\x83\x90\x1C\x17\x90U`\0\x82a\n\xABV[`\x02\x82\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x82 `\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x01U\x82a\n\xABV[a\x0B\x03\x82\x82a\x0B\x07V[PPV[`\x05\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x02\x85\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x95`\xE0\x95\x90\x95\x1B\x94`\0\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x0BxW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0BdW[PPPPP\x90P`\0\x83a\x0B\x8B\x83a\x0EgV[`@Q` \x01a\x0B\x9C\x92\x91\x90a AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01\x8B\x01` \x90\x81R\x83\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x83R\x81R\x92\x81 \x91\x94P\x90\x92\x90\x91a\x0B\xEE\x91\x86\x91\x88\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x0C&Wa\x0C$\x87a\x0F\x0EV[P[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x88\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x88\x16\x84R\x82R\x80\x83 \x90Q\x90\x91\x83\x91a\x0Ce\x91\x87\x91\x89\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T`\0\x1B\x90P`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0C\xAA\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x0C\xE5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xEAV[``\x91P[P\x91Pa\r\x03\x90P\x81a\x0C\xFE\x88` a \xDBV[a\x0F\x19V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90a\x1F\xC5V[\x90P\x80\x82\x14a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xAE\x90a \xF2V[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xCA\x10\xBB`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cp\xCA\x10\xBB\x90a\r\xF2\x90\x8B\x90\x87\x90\x8E\x90`\x04\x01a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x0CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E W=`\0\x80>=`\0\xFD[PPP`\x05\x8B\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP`\x03\x8A\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x0ES`\x02\x8B\x01`\0a\x1DXV[\x89`\x04\x01`\0\x90UPPPPPPPPPPV[```\0\x82Q` a\x0Ey\x91\x90a \xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x91Wa\x0E\x91a!\xAEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\xBBW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\x07W`\0\x84\x82\x81Q\x81\x10a\x0E\xDEWa\x0E\xDEa!\xC4V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x0E\xFF\x90a!\xDAV[\x91PPa\x0E\xC1V[P\x92\x91PPV[`\0a\n\xAE\x82a\x0F\x96V[`\0\x80`\0` \x85Q\x11a\x0F.W\x84Qa\x0F1V[` [\x90P`\0[\x81\x81\x10\x15a\x0F\x8CWa\x0FI\x81`\x08a \xDBV[\x86a\x0FT\x83\x88a\x1F\xF4V[\x81Q\x81\x10a\x0FdWa\x0Fda!\xC4V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x0F\x84\x81a!\xDAV[\x91PPa\x0F6V[P\x90\x94\x93PPPPV[`\x05\x81\x01T`\x03\x82\x01T`\x04\x83\x01T`\x02\x84\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\0\x96`\x01`\x01`\xA0\x1B\x03\x16\x95`\xE0\x1B\x94\x93\x87\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x10\x06W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0F\xF2W[PPP`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x95\x96P\x94\x91\x93Pa\x10P\x92P\x85\x91\x87\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16\x15a\x10\xECW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x10\xBC\x91\x85\x91\x87\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x94PPPPP\x91\x90PV[`\0\x83a\x10\xF8\x83a\x1CEV[`@Q` \x01a\x11\t\x92\x91\x90a AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c&l\xF1\t`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11fW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11zW=`\0\x80>=`\0\xFD[PPPP`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x11\x99\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x11\xD4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\xD9V[``\x91P[P\x91Pa\x11\xF2\x90P\x81a\x11\xED\x87` a \xDBV[a\x1C\xE5V[`@Qce\xBC\x94\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\x04\x82\x01R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ce\xBC\x94\x81\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12{\x91\x90\x81\x01\x90a\"\x8EV[P\x90P\x80Q`\x01\x03a\x15=W`\0`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x89\x84`\0\x81Q\x81\x10a\x12\xBDWa\x12\xBDa!\xC4V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xF6\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x137\x91\x90a\x1F\xC5V[\x90P\x80a\x13\x9EW\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x88\x83`\0\x81Q\x81\x10a\x13sWa\x13sa!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[\x80\x83\x14a\x13\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xAE\x90a \xF2V[\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x88\x88\x87\x89`@Q` \x01a\x13\xF3\x92\x91\x90a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`\0\x81Q\x81\x10a\x14\x1CWa\x14\x1Ca!\xC4V[` \x02` \x01\x01Q`\0\x1C`@Qa\x147\x94\x93\x92\x91\x90a\"\xF2V[`@Q\x80\x91\x03\x90\xA1\x81`\0\x81Q\x81\x10a\x14RWa\x14Ra!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R\x8C\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x14\x9B\x91\x8A\x91\x8C\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x82\x82\x01\x93\x90\x93R\x90\x82\x01`\0\x90\x81 \x93\x90\x93U`\x01`\x01`\xA0\x1B\x03\x8B\x16\x83R`\x01\x8D\x81\x01\x82R\x82\x84 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x85R\x82R\x82\x84 \x92Q\x90\x93\x91a\x15\x03\x91\x8A\x91\x8C\x91\x01a \xA1V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPa\x1A\xD0V[`\x01\x81Q\x11\x15a\x1A`W`\0[\x81Q\x81\x10\x15a\x1AZW`\0`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x8A\x85\x85\x81Q\x81\x10a\x15\x88Wa\x15\x88a!\xC4V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xC1\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x02\x91\x90a\x1F\xC5V[\x90P\x80a\x16hW\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x89\x84\x84\x81Q\x81\x10a\x16=Wa\x16=a!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8A\x85\x85\x81Q\x81\x10a\x16\x9BWa\x16\x9Ba!\xC4V[` \x02` \x01\x01Qa\x137`\xF0\x1B`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xC7\x93\x92\x91\x90a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xF5W=`\0\x80>=`\0\xFD[PPPP`\0``\x8A`\x01`\x01`\xA0\x1B\x03\x16\x87`@Qa\x17\x15\x91\x90a rV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x17PW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x17UV[``\x91P[P\x90\x92P\x90Pa\x17j\x81a\x11\xED\x8B` a \xDBV[\x95P\x81\x80\x15a\x17}WPa\x137`\xF0\x1B\x86\x14[\x15a\x19\xB8W\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x8B\x8B\x8A\x8C`@Q` \x01a\x17\xB8\x92\x91\x90a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88\x81Q\x81\x10a\x17\xE0Wa\x17\xE0a!\xC4V[` \x02` \x01\x01Q`\0\x1C`@Qa\x17\xFB\x94\x93\x92\x91\x90a\"\xF2V[`@Q\x80\x91\x03\x90\xA1\x84\x84\x81Q\x81\x10a\x18\x15Wa\x18\x15a!\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R\x8F\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8F\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x18^\x91\x8D\x91\x8F\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x8D`\x01\x01`\0\x8D`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C`\x01`\x01`\xE0\x1B\x03\x19\x16`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x8C`@Q` \x01a\x18\xE9\x92\x91\x90a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x19WWa\x19Wa!\xC4V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19~\x93\x92\x91\x90a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xACW=`\0\x80>=`\0\xFD[PPPPPPPa\x1AZV[`\0\x80Q` a##\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x19\xEBWa\x19\xEBa!\xC4V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x12\x93\x92\x91\x90a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A@W=`\0\x80>=`\0\xFD[PPPPPPP\x80\x80a\x1AR\x90a!\xDAV[\x91PPa\x15JV[Pa\x1A\xD0V[`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FstdStorage find(StdStorage): No `D\x82\x01R\x7Fstorage use detected for target.`d\x82\x01R`\x84\x01a\r\xAEV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1B\x12\x91\x88\x91\x8A\x91\x01a \xA1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x1B\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FstdStorage find(StdStorage): Slo`D\x82\x01Rn:\x149\x94\x9077\xBA\x1037\xBA\xB72\x17`\x89\x1B`d\x82\x01R`\x84\x01a\r\xAEV[`\x05\x89\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\x03\x89\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x1B\xCE`\x02\x8A\x01`\0a\x1DXV[`\0`\x04\x8A\x01\x81\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R` \x8A\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1C\x12\x91\x88\x91\x8A\x91\x01a \xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x97PPPPPPPP\x91\x90PV[```\0\x82Q` a\x1CW\x91\x90a \xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1CoWa\x1Coa!\xAEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1C\x99W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\x07W`\0\x84\x82\x81Q\x81\x10a\x1C\xBCWa\x1C\xBCa!\xC4V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x1C\xDD\x90a!\xDAV[\x91PPa\x1C\x9FV[`\0\x80`\0` \x85Q\x11a\x1C\xFAW\x84Qa\x1C\xFDV[` [\x90P`\0[\x81\x81\x10\x15a\x0F\x8CWa\x1D\x15\x81`\x08a \xDBV[\x86a\x1D \x83\x88a\x1F\xF4V[\x81Q\x81\x10a\x1D0Wa\x1D0a!\xC4V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x1DP\x81a!\xDAV[\x91PPa\x1D\x02V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x1Dv\x91\x90a\x1DyV[PV[[\x80\x82\x11\x15a\x1D\x8EW`\0\x81U`\x01\x01a\x1DzV[P\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1D\xD3W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1D\xAEV[P\x90\x96\x95PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x082W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\x0BW`\0\x80\xFD[a\x1E\x14\x84a\x1D\xDFV[\x92Pa\x1E\"` \x85\x01a\x1D\xDFV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x1E\xD6W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x1E\xC1W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x1E\x97V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x1EZV[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x1F\0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1E\xE8V[PP`\0\x91\x01RV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1FvW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra\x1FW\x81\x89\x89\x01\x8A\x85\x01a\x1E\xE5V[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1F0V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\x95W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\xAEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\xBEW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xD7W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n\xAEWa\n\xAEa\x1F\xDEV[`\x01\x81\x81\x1C\x90\x82\x16\x80a \x1BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a ;WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a d\x81`\x04\x85\x01` \x87\x01a\x1E\xE5V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa \x84\x81\x84` \x87\x01a\x1E\xE5V[\x91\x90\x91\x01\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\n\xAEWa\n\xAEa\x1F\xDEV[\x82Q`\0\x90\x82\x90` \x80\x87\x01\x84[\x83\x81\x10\x15a \xCBW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a \xAFV[PP\x94\x82RP\x90\x92\x01\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xAEWa\n\xAEa\x1F\xDEV[` \x80\x82R`o\x90\x82\x01R\x7FstdStorage find(StdStorage): Pac`@\x82\x01R\x7Fked slot. This would cause dange``\x82\x01R\x7Frous overwriting and currently i`\x80\x82\x01Rn9\xB7\x13\xBA\x109\xBA\xB887\xB9:2\xB2\x17`\x89\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a!\xECWa!\xECa\x1F\xDEV[P`\x01\x01\x90V[`\0\x82`\x1F\x83\x01\x12a\"\x04W`\0\x80\xFD[\x81Q` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x11\x15a\"!Wa\"!a!\xAEV[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15a\"FWa\"Fa!\xAEV[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15a\"dW`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15a\"\x83W\x81Q\x83R\x91\x83\x01\x91\x90\x83\x01\x90a\"jV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xA1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xB9W`\0\x80\xFD[a\"\xC5\x86\x83\x87\x01a!\xF3V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\"\xDBW`\0\x80\xFD[Pa\"\xE8\x85\x82\x86\x01a!\xF3V[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V\xFE\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \x91u\xC5\x9D/\xEC\xE1S\xC8\xFD\xE6\x8F\xC5\x8B\xB4p\t\x01=\x973+\xD7\xEC\x84S\x87\x8Dw\xE4&\x05dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKPARASWAPTOKENTRANSFERPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockParaSwapTokenTransferProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockParaSwapTokenTransferProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockParaSwapTokenTransferProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockParaSwapTokenTransferProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockParaSwapTokenTransferProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockParaSwapTokenTransferProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockParaSwapTokenTransferProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKPARASWAPTOKENTRANSFERPROXY_ABI.clone(),
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
                MOCKPARASWAPTOKENTRANSFERPROXY_ABI.clone(),
                MOCKPARASWAPTOKENTRANSFERPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeContracts` (0xe20c9f71) function
        pub fn exclude_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSenders` (0x1ed7831c) function
        pub fn exclude_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockSwap` (0x60154da9) function
        pub fn mock_swap(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 21, 77, 169], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setToAmount` (0xb11410af) function
        pub fn set_to_amount(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 20, 16, 175], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetContracts` (0x3f7286f4) function
        pub fn target_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSenders` (0x3e5e3c23) function
        pub fn target_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SlotFound` event
        pub fn slot_found_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SlotFoundFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WARNING_UninitedSlot` event
        pub fn warning_uninited_slot_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WarningUninitedSlotFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockParaSwapTokenTransferProxyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockParaSwapTokenTransferProxy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "SlotFound", abi = "SlotFound(address,bytes4,bytes32,uint256)")]
    pub struct SlotFoundFilter {
        pub who: ::ethers::core::types::Address,
        pub fsig: [u8; 4],
        pub keys_hash: [u8; 32],
        pub slot: ::ethers::core::types::U256,
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
        name = "WARNING_UninitedSlot",
        abi = "WARNING_UninitedSlot(address,uint256)"
    )]
    pub struct WarningUninitedSlotFilter {
        pub who: ::ethers::core::types::Address,
        pub slot: ::ethers::core::types::U256,
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
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
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
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
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
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
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
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
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's events
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
    pub enum MockParaSwapTokenTransferProxyEvents {
        SlotFoundFilter(SlotFoundFilter),
        WarningUninitedSlotFilter(WarningUninitedSlotFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockParaSwapTokenTransferProxyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = SlotFoundFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::SlotFoundFilter(decoded),
                );
            }
            if let Ok(decoded) = WarningUninitedSlotFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::WarningUninitedSlotFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(MockParaSwapTokenTransferProxyEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogArray1Filter(decoded),
                );
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogArray2Filter(decoded),
                );
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogArray3Filter(decoded),
                );
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(MockParaSwapTokenTransferProxyEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogBytes32Filter(decoded),
                );
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(MockParaSwapTokenTransferProxyEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedArray1Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedArray2Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedArray3Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedBytesFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedBytes32Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedDecimalIntFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedDecimalUintFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedIntFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogNamedUintFilter(decoded),
                );
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(
                    MockParaSwapTokenTransferProxyEvents::LogStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(MockParaSwapTokenTransferProxyEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(MockParaSwapTokenTransferProxyEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockParaSwapTokenTransferProxyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SlotFoundFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WarningUninitedSlotFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytes32Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalIntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SlotFoundFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: SlotFoundFilter) -> Self {
            Self::SlotFoundFilter(value)
        }
    }
    impl ::core::convert::From<WarningUninitedSlotFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: WarningUninitedSlotFilter) -> Self {
            Self::WarningUninitedSlotFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter>
    for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for MockParaSwapTokenTransferProxyEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    ///Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    ///Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `mockSwap` function with signature `mockSwap(address,address,uint256)` and selector `0x60154da9`
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
    #[ethcall(name = "mockSwap", abi = "mockSwap(address,address,uint256)")]
    pub struct MockSwapCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setToAmount` function with signature `setToAmount(uint256)` and selector `0xb11410af`
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
    #[ethcall(name = "setToAmount", abi = "setToAmount(uint256)")]
    pub struct SetToAmountCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    ///Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    ///Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    ///Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    ///Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
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
    pub enum MockParaSwapTokenTransferProxyCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        MockSwap(MockSwapCall),
        SetToAmount(SetToAmountCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockParaSwapTokenTransferProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) = <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) = <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <MockSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockSwap(decoded));
            }
            if let Ok(decoded) = <SetToAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetToAmount(decoded));
            }
            if let Ok(decoded) = <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) = <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) = <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) = <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSenders(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockParaSwapTokenTransferProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetToAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockParaSwapTokenTransferProxyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetToAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for MockParaSwapTokenTransferProxyCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for MockParaSwapTokenTransferProxyCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<MockSwapCall> for MockParaSwapTokenTransferProxyCalls {
        fn from(value: MockSwapCall) -> Self {
            Self::MockSwap(value)
        }
    }
    impl ::core::convert::From<SetToAmountCall> for MockParaSwapTokenTransferProxyCalls {
        fn from(value: SetToAmountCall) -> Self {
            Self::SetToAmount(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall>
    for MockParaSwapTokenTransferProxyCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///`FuzzSelector(address,bytes4[])`
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
    pub struct FuzzSelector {
        pub addr: ::ethers::core::types::Address,
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
}
