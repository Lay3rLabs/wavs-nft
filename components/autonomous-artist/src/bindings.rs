pub type TriggerAction = wavs::worker::layer_types::TriggerAction;
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_cabi<T: Guest>(arg0: *mut u8) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    let len2 = l1;
    let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
    let l3 = *arg0.add(8).cast::<*mut u8>();
    let l4 = *arg0.add(12).cast::<usize>();
    let len5 = l4;
    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
    let l6 = i32::from(*arg0.add(16).cast::<u8>());
    use wavs::worker::layer_types::TriggerSource as V26;
    let v26 = match l6 {
        0 => {
            let e26 = {
                let l7 = *arg0.add(20).cast::<*mut u8>();
                let l8 = *arg0.add(24).cast::<usize>();
                let len9 = l8;
                let l10 = *arg0.add(28).cast::<*mut u8>();
                let l11 = *arg0.add(32).cast::<usize>();
                let len12 = l11;
                let bytes12 = _rt::Vec::from_raw_parts(l10.cast(), len12, len12);
                let l13 = *arg0.add(36).cast::<*mut u8>();
                let l14 = *arg0.add(40).cast::<usize>();
                let len15 = l14;
                wavs::worker::layer_types::TriggerSourceEthContractEvent {
                    address: wavs::worker::layer_types::EthAddress {
                        raw_bytes: _rt::Vec::from_raw_parts(l7.cast(), len9, len9),
                    },
                    chain_name: _rt::string_lift(bytes12),
                    event_hash: _rt::Vec::from_raw_parts(l13.cast(), len15, len15),
                }
            };
            V26::EthContractEvent(e26)
        }
        1 => {
            let e26 = {
                let l16 = *arg0.add(20).cast::<*mut u8>();
                let l17 = *arg0.add(24).cast::<usize>();
                let len18 = l17;
                let bytes18 = _rt::Vec::from_raw_parts(l16.cast(), len18, len18);
                let l19 = *arg0.add(28).cast::<i32>();
                let l20 = *arg0.add(32).cast::<*mut u8>();
                let l21 = *arg0.add(36).cast::<usize>();
                let len22 = l21;
                let bytes22 = _rt::Vec::from_raw_parts(l20.cast(), len22, len22);
                let l23 = *arg0.add(40).cast::<*mut u8>();
                let l24 = *arg0.add(44).cast::<usize>();
                let len25 = l24;
                let bytes25 = _rt::Vec::from_raw_parts(l23.cast(), len25, len25);
                wavs::worker::layer_types::TriggerSourceCosmosContractEvent {
                    address: wavs::worker::layer_types::CosmosAddress {
                        bech32_addr: _rt::string_lift(bytes18),
                        prefix_len: l19 as u32,
                    },
                    chain_name: _rt::string_lift(bytes22),
                    event_type: _rt::string_lift(bytes25),
                }
            };
            V26::CosmosContractEvent(e26)
        }
        n => {
            debug_assert_eq!(n, 2, "invalid enum discriminant");
            V26::Manual
        }
    };
    let l27 = i32::from(*arg0.add(48).cast::<u8>());
    use wavs::worker::layer_types::TriggerData as V67;
    let v67 = match l27 {
        0 => {
            let e67 = {
                let l28 = *arg0.add(56).cast::<*mut u8>();
                let l29 = *arg0.add(60).cast::<usize>();
                let len30 = l29;
                let l31 = *arg0.add(64).cast::<*mut u8>();
                let l32 = *arg0.add(68).cast::<usize>();
                let len33 = l32;
                let bytes33 = _rt::Vec::from_raw_parts(l31.cast(), len33, len33);
                let l34 = *arg0.add(72).cast::<*mut u8>();
                let l35 = *arg0.add(76).cast::<usize>();
                let base39 = l34;
                let len39 = l35;
                let mut result39 = _rt::Vec::with_capacity(len39);
                for i in 0..len39 {
                    let base = base39.add(i * 8);
                    let e39 = {
                        let l36 = *base.add(0).cast::<*mut u8>();
                        let l37 = *base.add(4).cast::<usize>();
                        let len38 = l37;
                        _rt::Vec::from_raw_parts(l36.cast(), len38, len38)
                    };
                    result39.push(e39);
                }
                _rt::cabi_dealloc(base39, len39 * 8, 4);
                let l40 = *arg0.add(80).cast::<*mut u8>();
                let l41 = *arg0.add(84).cast::<usize>();
                let len42 = l41;
                let l43 = *arg0.add(88).cast::<i64>();
                wavs::worker::layer_types::TriggerDataEthContractEvent {
                    contract_address: wavs::worker::layer_types::EthAddress {
                        raw_bytes: _rt::Vec::from_raw_parts(l28.cast(), len30, len30),
                    },
                    chain_name: _rt::string_lift(bytes33),
                    log: wavs::worker::layer_types::EthEventLogData {
                        topics: result39,
                        data: _rt::Vec::from_raw_parts(l40.cast(), len42, len42),
                    },
                    block_height: l43 as u64,
                }
            };
            V67::EthContractEvent(e67)
        }
        1 => {
            let e67 = {
                let l44 = *arg0.add(56).cast::<*mut u8>();
                let l45 = *arg0.add(60).cast::<usize>();
                let len46 = l45;
                let bytes46 = _rt::Vec::from_raw_parts(l44.cast(), len46, len46);
                let l47 = *arg0.add(64).cast::<i32>();
                let l48 = *arg0.add(68).cast::<*mut u8>();
                let l49 = *arg0.add(72).cast::<usize>();
                let len50 = l49;
                let bytes50 = _rt::Vec::from_raw_parts(l48.cast(), len50, len50);
                let l51 = *arg0.add(76).cast::<*mut u8>();
                let l52 = *arg0.add(80).cast::<usize>();
                let len53 = l52;
                let bytes53 = _rt::Vec::from_raw_parts(l51.cast(), len53, len53);
                let l54 = *arg0.add(84).cast::<*mut u8>();
                let l55 = *arg0.add(88).cast::<usize>();
                let base62 = l54;
                let len62 = l55;
                let mut result62 = _rt::Vec::with_capacity(len62);
                for i in 0..len62 {
                    let base = base62.add(i * 16);
                    let e62 = {
                        let l56 = *base.add(0).cast::<*mut u8>();
                        let l57 = *base.add(4).cast::<usize>();
                        let len58 = l57;
                        let bytes58 = _rt::Vec::from_raw_parts(l56.cast(), len58, len58);
                        let l59 = *base.add(8).cast::<*mut u8>();
                        let l60 = *base.add(12).cast::<usize>();
                        let len61 = l60;
                        let bytes61 = _rt::Vec::from_raw_parts(l59.cast(), len61, len61);
                        (_rt::string_lift(bytes58), _rt::string_lift(bytes61))
                    };
                    result62.push(e62);
                }
                _rt::cabi_dealloc(base62, len62 * 16, 4);
                let l63 = *arg0.add(96).cast::<i64>();
                wavs::worker::layer_types::TriggerDataCosmosContractEvent {
                    contract_address: wavs::worker::layer_types::CosmosAddress {
                        bech32_addr: _rt::string_lift(bytes46),
                        prefix_len: l47 as u32,
                    },
                    chain_name: _rt::string_lift(bytes50),
                    event: wavs::worker::layer_types::CosmosEvent {
                        ty: _rt::string_lift(bytes53),
                        attributes: result62,
                    },
                    block_height: l63 as u64,
                }
            };
            V67::CosmosContractEvent(e67)
        }
        n => {
            debug_assert_eq!(n, 2, "invalid enum discriminant");
            let e67 = {
                let l64 = *arg0.add(56).cast::<*mut u8>();
                let l65 = *arg0.add(60).cast::<usize>();
                let len66 = l65;
                _rt::Vec::from_raw_parts(l64.cast(), len66, len66)
            };
            V67::Raw(e67)
        }
    };
    let result68 = T::run(wavs::worker::layer_types::TriggerAction {
        config: wavs::worker::layer_types::TriggerConfig {
            service_id: _rt::string_lift(bytes2),
            workflow_id: _rt::string_lift(bytes5),
            trigger_source: v26,
        },
        data: v67,
    });
    _rt::cabi_dealloc(arg0, 104, 8);
    let ptr69 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result68 {
        Ok(e) => {
            *ptr69.add(0).cast::<u8>() = (0i32) as u8;
            match e {
                Some(e) => {
                    *ptr69.add(4).cast::<u8>() = (1i32) as u8;
                    let vec70 = (e).into_boxed_slice();
                    let ptr70 = vec70.as_ptr().cast::<u8>();
                    let len70 = vec70.len();
                    ::core::mem::forget(vec70);
                    *ptr69.add(12).cast::<usize>() = len70;
                    *ptr69.add(8).cast::<*mut u8>() = ptr70.cast_mut();
                }
                None => {
                    *ptr69.add(4).cast::<u8>() = (0i32) as u8;
                }
            };
        }
        Err(e) => {
            *ptr69.add(0).cast::<u8>() = (1i32) as u8;
            let vec71 = (e.into_bytes()).into_boxed_slice();
            let ptr71 = vec71.as_ptr().cast::<u8>();
            let len71 = vec71.len();
            ::core::mem::forget(vec71);
            *ptr69.add(8).cast::<usize>() = len71;
            *ptr69.add(4).cast::<*mut u8>() = ptr71.cast_mut();
        }
    };
    ptr69
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = i32::from(*arg0.add(4).cast::<u8>());
            match l1 {
                0 => {}
                _ => {
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    let base4 = l2;
                    let len4 = l3;
                    _rt::cabi_dealloc(base4, len4 * 1, 1);
                }
            }
        }
        _ => {
            let l5 = *arg0.add(4).cast::<*mut u8>();
            let l6 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l5, l6, 1);
        }
    }
}
pub trait Guest {
    fn run(trigger_action: TriggerAction) -> Result<Option<_rt::Vec<u8>>, _rt::String>;
}
#[doc(hidden)]
macro_rules! __export_world_layer_trigger_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "run"] unsafe extern "C" fn export_run(arg0 : *
        mut u8,) -> * mut u8 { $($path_to_types)*:: _export_run_cabi::<$ty > (arg0) }
        #[export_name = "cabi_post_run"] unsafe extern "C" fn _post_return_run(arg0 : *
        mut u8,) { $($path_to_types)*:: __post_return_run::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_layer_trigger_world_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);
#[allow(dead_code)]
pub mod wavs {
    #[allow(dead_code)]
    pub mod worker {
        #[allow(dead_code, clippy::all)]
        pub mod layer_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub struct CosmosAddress {
                pub bech32_addr: _rt::String,
                /// prefix is the first part of the bech32 address
                pub prefix_len: u32,
            }
            impl ::core::fmt::Debug for CosmosAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("CosmosAddress")
                        .field("bech32-addr", &self.bech32_addr)
                        .field("prefix-len", &self.prefix_len)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct CosmosEvent {
                pub ty: _rt::String,
                pub attributes: _rt::Vec<(_rt::String, _rt::String)>,
            }
            impl ::core::fmt::Debug for CosmosEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("CosmosEvent")
                        .field("ty", &self.ty)
                        .field("attributes", &self.attributes)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct CosmosChainConfig {
                pub chain_id: _rt::String,
                pub rpc_endpoint: Option<_rt::String>,
                pub grpc_endpoint: Option<_rt::String>,
                pub grpc_web_endpoint: Option<_rt::String>,
                pub gas_price: f32,
                pub gas_denom: _rt::String,
                pub bech32_prefix: _rt::String,
            }
            impl ::core::fmt::Debug for CosmosChainConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("CosmosChainConfig")
                        .field("chain-id", &self.chain_id)
                        .field("rpc-endpoint", &self.rpc_endpoint)
                        .field("grpc-endpoint", &self.grpc_endpoint)
                        .field("grpc-web-endpoint", &self.grpc_web_endpoint)
                        .field("gas-price", &self.gas_price)
                        .field("gas-denom", &self.gas_denom)
                        .field("bech32-prefix", &self.bech32_prefix)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct EthAddress {
                pub raw_bytes: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for EthAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("EthAddress").field("raw-bytes", &self.raw_bytes).finish()
                }
            }
            #[derive(Clone)]
            pub struct EthEventLogData {
                /// the raw log topics that can be decoded into an event
                pub topics: _rt::Vec<_rt::Vec<u8>>,
                /// the raw log data that can be decoded into an event
                pub data: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for EthEventLogData {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("EthEventLogData")
                        .field("topics", &self.topics)
                        .field("data", &self.data)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct EthChainConfig {
                pub chain_id: _rt::String,
                pub ws_endpoint: Option<_rt::String>,
                pub http_endpoint: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for EthChainConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("EthChainConfig")
                        .field("chain-id", &self.chain_id)
                        .field("ws-endpoint", &self.ws_endpoint)
                        .field("http-endpoint", &self.http_endpoint)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerSourceEthContractEvent {
                pub address: EthAddress,
                pub chain_name: _rt::String,
                pub event_hash: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for TriggerSourceEthContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerSourceEthContractEvent")
                        .field("address", &self.address)
                        .field("chain-name", &self.chain_name)
                        .field("event-hash", &self.event_hash)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerSourceCosmosContractEvent {
                pub address: CosmosAddress,
                pub chain_name: _rt::String,
                pub event_type: _rt::String,
            }
            impl ::core::fmt::Debug for TriggerSourceCosmosContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerSourceCosmosContractEvent")
                        .field("address", &self.address)
                        .field("chain-name", &self.chain_name)
                        .field("event-type", &self.event_type)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum TriggerSource {
                EthContractEvent(TriggerSourceEthContractEvent),
                CosmosContractEvent(TriggerSourceCosmosContractEvent),
                Manual,
            }
            impl ::core::fmt::Debug for TriggerSource {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        TriggerSource::EthContractEvent(e) => {
                            f.debug_tuple("TriggerSource::EthContractEvent").field(e).finish()
                        }
                        TriggerSource::CosmosContractEvent(e) => {
                            f.debug_tuple("TriggerSource::CosmosContractEvent").field(e).finish()
                        }
                        TriggerSource::Manual => f.debug_tuple("TriggerSource::Manual").finish(),
                    }
                }
            }
            #[derive(Clone)]
            pub struct TriggerConfig {
                pub service_id: _rt::String,
                pub workflow_id: _rt::String,
                pub trigger_source: TriggerSource,
            }
            impl ::core::fmt::Debug for TriggerConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerConfig")
                        .field("service-id", &self.service_id)
                        .field("workflow-id", &self.workflow_id)
                        .field("trigger-source", &self.trigger_source)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerDataEthContractEvent {
                pub contract_address: EthAddress,
                pub chain_name: _rt::String,
                pub log: EthEventLogData,
                pub block_height: u64,
            }
            impl ::core::fmt::Debug for TriggerDataEthContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerDataEthContractEvent")
                        .field("contract-address", &self.contract_address)
                        .field("chain-name", &self.chain_name)
                        .field("log", &self.log)
                        .field("block-height", &self.block_height)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerDataCosmosContractEvent {
                pub contract_address: CosmosAddress,
                pub chain_name: _rt::String,
                pub event: CosmosEvent,
                pub block_height: u64,
            }
            impl ::core::fmt::Debug for TriggerDataCosmosContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerDataCosmosContractEvent")
                        .field("contract-address", &self.contract_address)
                        .field("chain-name", &self.chain_name)
                        .field("event", &self.event)
                        .field("block-height", &self.block_height)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum TriggerData {
                EthContractEvent(TriggerDataEthContractEvent),
                CosmosContractEvent(TriggerDataCosmosContractEvent),
                Raw(_rt::Vec<u8>),
            }
            impl ::core::fmt::Debug for TriggerData {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        TriggerData::EthContractEvent(e) => {
                            f.debug_tuple("TriggerData::EthContractEvent").field(e).finish()
                        }
                        TriggerData::CosmosContractEvent(e) => {
                            f.debug_tuple("TriggerData::CosmosContractEvent").field(e).finish()
                        }
                        TriggerData::Raw(e) => f.debug_tuple("TriggerData::Raw").field(e).finish(),
                    }
                }
            }
            #[derive(Clone)]
            pub struct TriggerAction {
                pub config: TriggerConfig,
                pub data: TriggerData,
            }
            impl ::core::fmt::Debug for TriggerAction {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerAction")
                        .field("config", &self.config)
                        .field("data", &self.data)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum LogLevel {
                Error,
                Warn,
                Info,
                Debug,
                Trace,
            }
            impl ::core::fmt::Debug for LogLevel {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        LogLevel::Error => f.debug_tuple("LogLevel::Error").finish(),
                        LogLevel::Warn => f.debug_tuple("LogLevel::Warn").finish(),
                        LogLevel::Info => f.debug_tuple("LogLevel::Info").finish(),
                        LogLevel::Debug => f.debug_tuple("LogLevel::Debug").finish(),
                        LogLevel::Trace => f.debug_tuple("LogLevel::Trace").finish(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code, clippy::all)]
pub mod host {
    #[used]
    #[doc(hidden)]
    static __FORCE_SECTION_REF: fn() = super::__link_custom_section_describing_imports;
    use super::_rt;
    pub type EthChainConfig = super::wavs::worker::layer_types::EthChainConfig;
    pub type CosmosChainConfig = super::wavs::worker::layer_types::CosmosChainConfig;
    pub type LogLevel = super::wavs::worker::layer_types::LogLevel;
    #[allow(unused_unsafe, clippy::all)]
    pub fn get_eth_chain_config(chain_name: &str) -> Option<EthChainConfig> {
        unsafe {
            #[repr(align(4))]
            struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
            let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 36]);
            let vec0 = chain_name;
            let ptr0 = vec0.as_ptr().cast::<u8>();
            let len0 = vec0.len();
            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "get-eth-chain-config"]
                fn wit_import(_: *mut u8, _: usize, _: *mut u8);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                unreachable!()
            }
            wit_import(ptr0.cast_mut(), len0, ptr1);
            let l2 = i32::from(*ptr1.add(0).cast::<u8>());
            match l2 {
                0 => None,
                1 => {
                    let e = {
                        let l3 = *ptr1.add(4).cast::<*mut u8>();
                        let l4 = *ptr1.add(8).cast::<usize>();
                        let len5 = l4;
                        let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                        let l6 = i32::from(*ptr1.add(12).cast::<u8>());
                        let l10 = i32::from(*ptr1.add(24).cast::<u8>());
                        super::wavs::worker::layer_types::EthChainConfig {
                            chain_id: _rt::string_lift(bytes5),
                            ws_endpoint: match l6 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l7 = *ptr1.add(16).cast::<*mut u8>();
                                        let l8 = *ptr1.add(20).cast::<usize>();
                                        let len9 = l8;
                                        let bytes9 =
                                            _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                                        _rt::string_lift(bytes9)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            http_endpoint: match l10 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l11 = *ptr1.add(28).cast::<*mut u8>();
                                        let l12 = *ptr1.add(32).cast::<usize>();
                                        let len13 = l12;
                                        let bytes13 =
                                            _rt::Vec::from_raw_parts(l11.cast(), len13, len13);
                                        _rt::string_lift(bytes13)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                        }
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            }
        }
    }
    #[allow(unused_unsafe, clippy::all)]
    pub fn get_cosmos_chain_config(chain_name: &str) -> Option<CosmosChainConfig> {
        unsafe {
            #[repr(align(4))]
            struct RetArea([::core::mem::MaybeUninit<u8>; 68]);
            let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 68]);
            let vec0 = chain_name;
            let ptr0 = vec0.as_ptr().cast::<u8>();
            let len0 = vec0.len();
            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "get-cosmos-chain-config"]
                fn wit_import(_: *mut u8, _: usize, _: *mut u8);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                unreachable!()
            }
            wit_import(ptr0.cast_mut(), len0, ptr1);
            let l2 = i32::from(*ptr1.add(0).cast::<u8>());
            match l2 {
                0 => None,
                1 => {
                    let e = {
                        let l3 = *ptr1.add(4).cast::<*mut u8>();
                        let l4 = *ptr1.add(8).cast::<usize>();
                        let len5 = l4;
                        let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                        let l6 = i32::from(*ptr1.add(12).cast::<u8>());
                        let l10 = i32::from(*ptr1.add(24).cast::<u8>());
                        let l14 = i32::from(*ptr1.add(36).cast::<u8>());
                        let l18 = *ptr1.add(48).cast::<f32>();
                        let l19 = *ptr1.add(52).cast::<*mut u8>();
                        let l20 = *ptr1.add(56).cast::<usize>();
                        let len21 = l20;
                        let bytes21 = _rt::Vec::from_raw_parts(l19.cast(), len21, len21);
                        let l22 = *ptr1.add(60).cast::<*mut u8>();
                        let l23 = *ptr1.add(64).cast::<usize>();
                        let len24 = l23;
                        let bytes24 = _rt::Vec::from_raw_parts(l22.cast(), len24, len24);
                        super::wavs::worker::layer_types::CosmosChainConfig {
                            chain_id: _rt::string_lift(bytes5),
                            rpc_endpoint: match l6 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l7 = *ptr1.add(16).cast::<*mut u8>();
                                        let l8 = *ptr1.add(20).cast::<usize>();
                                        let len9 = l8;
                                        let bytes9 =
                                            _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                                        _rt::string_lift(bytes9)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            grpc_endpoint: match l10 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l11 = *ptr1.add(28).cast::<*mut u8>();
                                        let l12 = *ptr1.add(32).cast::<usize>();
                                        let len13 = l12;
                                        let bytes13 =
                                            _rt::Vec::from_raw_parts(l11.cast(), len13, len13);
                                        _rt::string_lift(bytes13)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            grpc_web_endpoint: match l14 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l15 = *ptr1.add(40).cast::<*mut u8>();
                                        let l16 = *ptr1.add(44).cast::<usize>();
                                        let len17 = l16;
                                        let bytes17 =
                                            _rt::Vec::from_raw_parts(l15.cast(), len17, len17);
                                        _rt::string_lift(bytes17)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            gas_price: l18,
                            gas_denom: _rt::string_lift(bytes21),
                            bech32_prefix: _rt::string_lift(bytes24),
                        }
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            }
        }
    }
    #[allow(unused_unsafe, clippy::all)]
    pub fn log(level: LogLevel, message: &str) {
        unsafe {
            use super::wavs::worker::layer_types::LogLevel as V0;
            let result1 = match level {
                V0::Error => 0i32,
                V0::Warn => 1i32,
                V0::Info => 2i32,
                V0::Debug => 3i32,
                V0::Trace => 4i32,
            };
            let vec2 = message;
            let ptr2 = vec2.as_ptr().cast::<u8>();
            let len2 = vec2.len();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "log"]
                fn wit_import(_: i32, _: *mut u8, _: usize);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: *mut u8, _: usize) {
                unreachable!()
            }
            wit_import(result1, ptr2.cast_mut(), len2);
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_layer_trigger_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_layer_trigger_world_cabi!($ty
        with_types_in $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_layer_trigger_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:wavs:worker@0.3.0:layer-trigger-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1580] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa2\x0b\x01A\x02\x01\
A\x0e\x01B#\x01r\x02\x0bbech32-addrs\x0aprefix-leny\x04\0\x0ecosmos-address\x03\0\
\0\x01o\x02ss\x01p\x02\x01r\x02\x02tys\x0aattributes\x03\x04\0\x0ccosmos-event\x03\
\0\x04\x01ks\x01r\x07\x08chain-ids\x0crpc-endpoint\x06\x0dgrpc-endpoint\x06\x11g\
rpc-web-endpoint\x06\x09gas-pricev\x09gas-denoms\x0dbech32-prefixs\x04\0\x13cosm\
os-chain-config\x03\0\x07\x01p}\x01r\x01\x09raw-bytes\x09\x04\0\x0beth-address\x03\
\0\x0a\x01p\x09\x01r\x02\x06topics\x0c\x04data\x09\x04\0\x12eth-event-log-data\x03\
\0\x0d\x01r\x03\x08chain-ids\x0bws-endpoint\x06\x0dhttp-endpoint\x06\x04\0\x10et\
h-chain-config\x03\0\x0f\x01r\x03\x07address\x0b\x0achain-names\x0aevent-hash\x09\
\x04\0!trigger-source-eth-contract-event\x03\0\x11\x01r\x03\x07address\x01\x0ach\
ain-names\x0aevent-types\x04\0$trigger-source-cosmos-contract-event\x03\0\x13\x01\
q\x03\x12eth-contract-event\x01\x12\0\x15cosmos-contract-event\x01\x14\0\x06manu\
al\0\0\x04\0\x0etrigger-source\x03\0\x15\x01r\x03\x0aservice-ids\x0bworkflow-ids\
\x0etrigger-source\x16\x04\0\x0etrigger-config\x03\0\x17\x01r\x04\x10contract-ad\
dress\x0b\x0achain-names\x03log\x0e\x0cblock-heightw\x04\0\x1ftrigger-data-eth-c\
ontract-event\x03\0\x19\x01r\x04\x10contract-address\x01\x0achain-names\x05event\
\x05\x0cblock-heightw\x04\0\"trigger-data-cosmos-contract-event\x03\0\x1b\x01q\x03\
\x12eth-contract-event\x01\x1a\0\x15cosmos-contract-event\x01\x1c\0\x03raw\x01\x09\
\0\x04\0\x0ctrigger-data\x03\0\x1d\x01r\x02\x06config\x18\x04data\x1e\x04\0\x0et\
rigger-action\x03\0\x1f\x01q\x05\x05error\0\0\x04warn\0\0\x04info\0\0\x05debug\0\
\0\x05trace\0\0\x04\0\x09log-level\x03\0!\x03\0\x1dwavs:worker/layer-types@0.3.0\
\x05\0\x02\x03\0\0\x0etrigger-action\x03\0\x0etrigger-action\x03\0\x01\x02\x03\0\
\0\x10eth-chain-config\x02\x03\0\0\x13cosmos-chain-config\x02\x03\0\0\x09log-lev\
el\x01B\x0e\x02\x03\x02\x01\x03\x04\0\x10eth-chain-config\x03\0\0\x02\x03\x02\x01\
\x04\x04\0\x13cosmos-chain-config\x03\0\x02\x02\x03\x02\x01\x05\x04\0\x09log-lev\
el\x03\0\x04\x01k\x01\x01@\x01\x0achain-names\0\x06\x04\0\x14get-eth-chain-confi\
g\x01\x07\x01k\x03\x01@\x01\x0achain-names\0\x08\x04\0\x17get-cosmos-chain-confi\
g\x01\x09\x01@\x02\x05level\x05\x07messages\x01\0\x04\0\x03log\x01\x0a\x03\0\x04\
host\x05\x06\x01p}\x01k\x07\x01j\x01\x08\x01s\x01@\x01\x0etrigger-action\x02\0\x09\
\x04\0\x03run\x01\x0a\x04\0%wavs:worker/layer-trigger-world@0.3.0\x04\0\x0b\x19\x01\
\0\x13layer-trigger-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit\
-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
