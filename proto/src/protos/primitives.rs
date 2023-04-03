/// / Balance type used in chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
/// / BlockNumber type used in chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockNumber {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
/// / AccountId types used on chain
/// TODO:
/// might be better to use PublicKey instead of AccountId?
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
