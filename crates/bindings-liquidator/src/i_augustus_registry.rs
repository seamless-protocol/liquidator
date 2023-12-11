pub use i_augustus_registry::*;
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
pub mod i_augustus_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addAugustus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAugustus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("augustus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isLatest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("banAugustus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("banAugustus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("augustus"),
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
                    ::std::borrow::ToOwned::to_owned("getAugustusByVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAugustusByVersion",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("getAugustusCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAugustusCount"),
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
                    ::std::borrow::ToOwned::to_owned("getLatestAugustus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLatestAugustus"),
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
                    ::std::borrow::ToOwned::to_owned("getLatestVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLatestVersion"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isValidAugustus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidAugustus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("augustus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IAUGUSTUSREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IAugustusRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAugustusRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAugustusRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAugustusRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAugustusRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IAugustusRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAugustusRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IAUGUSTUSREGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addAugustus` (0xb552d88e) function
        pub fn add_augustus(
            &self,
            version: ::std::string::String,
            augustus: ::ethers::core::types::Address,
            is_latest: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 82, 216, 142], (version, augustus, is_latest))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `banAugustus` (0x82709a71) function
        pub fn ban_augustus(
            &self,
            augustus: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 112, 154, 113], augustus)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAugustusByVersion` (0xe2b40118) function
        pub fn get_augustus_by_version(
            &self,
            version: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([226, 180, 1, 24], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAugustusCount` (0x61172b71) function
        pub fn get_augustus_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 23, 43, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestAugustus` (0xae851d85) function
        pub fn get_latest_augustus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([174, 133, 29, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestVersion` (0x0e6d1de9) function
        pub fn get_latest_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([14, 109, 29, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidAugustus` (0xfb04e17b) function
        pub fn is_valid_augustus(
            &self,
            augustus: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([251, 4, 225, 123], augustus)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IAugustusRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addAugustus` function with signature `addAugustus(string,address,bool)` and selector `0xb552d88e`
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
    #[ethcall(name = "addAugustus", abi = "addAugustus(string,address,bool)")]
    pub struct AddAugustusCall {
        pub version: ::std::string::String,
        pub augustus: ::ethers::core::types::Address,
        pub is_latest: bool,
    }
    ///Container type for all input parameters for the `banAugustus` function with signature `banAugustus(address)` and selector `0x82709a71`
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
    #[ethcall(name = "banAugustus", abi = "banAugustus(address)")]
    pub struct BanAugustusCall {
        pub augustus: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAugustusByVersion` function with signature `getAugustusByVersion(string)` and selector `0xe2b40118`
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
    #[ethcall(name = "getAugustusByVersion", abi = "getAugustusByVersion(string)")]
    pub struct GetAugustusByVersionCall {
        pub version: ::std::string::String,
    }
    ///Container type for all input parameters for the `getAugustusCount` function with signature `getAugustusCount()` and selector `0x61172b71`
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
    #[ethcall(name = "getAugustusCount", abi = "getAugustusCount()")]
    pub struct GetAugustusCountCall;
    ///Container type for all input parameters for the `getLatestAugustus` function with signature `getLatestAugustus()` and selector `0xae851d85`
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
    #[ethcall(name = "getLatestAugustus", abi = "getLatestAugustus()")]
    pub struct GetLatestAugustusCall;
    ///Container type for all input parameters for the `getLatestVersion` function with signature `getLatestVersion()` and selector `0x0e6d1de9`
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
    #[ethcall(name = "getLatestVersion", abi = "getLatestVersion()")]
    pub struct GetLatestVersionCall;
    ///Container type for all input parameters for the `isValidAugustus` function with signature `isValidAugustus(address)` and selector `0xfb04e17b`
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
    #[ethcall(name = "isValidAugustus", abi = "isValidAugustus(address)")]
    pub struct IsValidAugustusCall {
        pub augustus: ::ethers::core::types::Address,
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
    pub enum IAugustusRegistryCalls {
        AddAugustus(AddAugustusCall),
        BanAugustus(BanAugustusCall),
        GetAugustusByVersion(GetAugustusByVersionCall),
        GetAugustusCount(GetAugustusCountCall),
        GetLatestAugustus(GetLatestAugustusCall),
        GetLatestVersion(GetLatestVersionCall),
        IsValidAugustus(IsValidAugustusCall),
    }
    impl ::ethers::core::abi::AbiDecode for IAugustusRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddAugustusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAugustus(decoded));
            }
            if let Ok(decoded) = <BanAugustusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BanAugustus(decoded));
            }
            if let Ok(decoded) = <GetAugustusByVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAugustusByVersion(decoded));
            }
            if let Ok(decoded) = <GetAugustusCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAugustusCount(decoded));
            }
            if let Ok(decoded) = <GetLatestAugustusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestAugustus(decoded));
            }
            if let Ok(decoded) = <GetLatestVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestVersion(decoded));
            }
            if let Ok(decoded) = <IsValidAugustusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsValidAugustus(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAugustusRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAugustus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BanAugustus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAugustusByVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAugustusCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestAugustus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidAugustus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAugustusRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAugustus(element) => ::core::fmt::Display::fmt(element, f),
                Self::BanAugustus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAugustusByVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAugustusCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLatestAugustus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLatestVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidAugustus(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAugustusCall> for IAugustusRegistryCalls {
        fn from(value: AddAugustusCall) -> Self {
            Self::AddAugustus(value)
        }
    }
    impl ::core::convert::From<BanAugustusCall> for IAugustusRegistryCalls {
        fn from(value: BanAugustusCall) -> Self {
            Self::BanAugustus(value)
        }
    }
    impl ::core::convert::From<GetAugustusByVersionCall> for IAugustusRegistryCalls {
        fn from(value: GetAugustusByVersionCall) -> Self {
            Self::GetAugustusByVersion(value)
        }
    }
    impl ::core::convert::From<GetAugustusCountCall> for IAugustusRegistryCalls {
        fn from(value: GetAugustusCountCall) -> Self {
            Self::GetAugustusCount(value)
        }
    }
    impl ::core::convert::From<GetLatestAugustusCall> for IAugustusRegistryCalls {
        fn from(value: GetLatestAugustusCall) -> Self {
            Self::GetLatestAugustus(value)
        }
    }
    impl ::core::convert::From<GetLatestVersionCall> for IAugustusRegistryCalls {
        fn from(value: GetLatestVersionCall) -> Self {
            Self::GetLatestVersion(value)
        }
    }
    impl ::core::convert::From<IsValidAugustusCall> for IAugustusRegistryCalls {
        fn from(value: IsValidAugustusCall) -> Self {
            Self::IsValidAugustus(value)
        }
    }
    ///Container type for all return fields from the `getAugustusByVersion` function with signature `getAugustusByVersion(string)` and selector `0xe2b40118`
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
    pub struct GetAugustusByVersionReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAugustusCount` function with signature `getAugustusCount()` and selector `0x61172b71`
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
    pub struct GetAugustusCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLatestAugustus` function with signature `getLatestAugustus()` and selector `0xae851d85`
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
    pub struct GetLatestAugustusReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLatestVersion` function with signature `getLatestVersion()` and selector `0x0e6d1de9`
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
    pub struct GetLatestVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `isValidAugustus` function with signature `isValidAugustus(address)` and selector `0xfb04e17b`
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
    pub struct IsValidAugustusReturn(pub bool);
}
