use neutron_std_derive::CosmwasmExt;
/// *
/// ExistenceProof takes a key and a value and a set of steps to perform on it.
/// The result of peforming all these steps will provide a "root hash", which can
/// be compared to the value in a header.
///
/// Since it is computationally infeasible to produce a hash collission for any of the used
/// cryptographic hash functions, if someone can provide a series of operations to transform
/// a given key and value into a root hash that matches some trusted root, these key and values
/// must be in the referenced merkle tree.
///
/// The only possible issue is maliablity in LeafOp, such as providing extra prefix data,
/// which should be controlled by a spec. Eg. with lengthOp as NONE,
/// prefix = FOO, key = BAR, value = CHOICE
/// and
/// prefix = F, key = OOBAR, value = CHOICE
/// would produce the same value.
///
/// With LengthOp this is tricker but not impossible. Which is why the "leafPrefixEqual" field
/// in the ProofSpec is valuable to prevent this mutability. And why all trees should
/// length-prefix the data before hashing it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.ExistenceProof")]
pub struct ExistenceProof {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub leaf: ::core::option::Option<LeafOp>,
    #[prost(message, repeated, tag = "4")]
    pub path: ::prost::alloc::vec::Vec<InnerOp>,
}
///
/// NonExistenceProof takes a proof of two neighbors, one left of the desired key,
/// one right of the desired key. If both proofs are valid AND they are neighbors,
/// then there is no valid proof for the given key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.NonExistenceProof")]
pub struct NonExistenceProof {
    /// TODO: remove this as unnecessary??? we prove a range
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub left: ::core::option::Option<ExistenceProof>,
    #[prost(message, optional, tag = "3")]
    pub right: ::core::option::Option<ExistenceProof>,
}
///
/// CommitmentProof is either an ExistenceProof or a NonExistenceProof, or a Batch of such messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.CommitmentProof")]
pub struct CommitmentProof {
    #[prost(oneof = "commitment_proof::Proof", tags = "1, 2, 3, 4")]
    pub proof: ::core::option::Option<commitment_proof::Proof>,
}
/// Nested message and enum types in `CommitmentProof`.
pub mod commitment_proof {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Proof {
        #[prost(message, tag = "1")]
        Exist(super::ExistenceProof),
        #[prost(message, tag = "2")]
        Nonexist(super::NonExistenceProof),
        #[prost(message, tag = "3")]
        Batch(super::BatchProof),
        #[prost(message, tag = "4")]
        Compressed(super::CompressedBatchProof),
    }
}
/// *
/// LeafOp represents the raw key-value data we wish to prove, and
/// must be flexible to represent the internal transformation from
/// the original key-value pairs into the basis hash, for many existing
/// merkle trees.
///
/// key and value are passed in. So that the signature of this operation is:
/// leafOp(key, value) -> output
///
/// To process this, first prehash the keys and values if needed (ANY means no hash in this case):
/// hkey = prehashKey(key)
/// hvalue = prehashValue(value)
///
/// Then combine the bytes, and hash it
/// output = hash(prefix || length(hkey) || hkey || length(hvalue) || hvalue)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.LeafOp")]
pub struct LeafOp {
    #[prost(enumeration = "HashOp", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub hash: i32,
    #[prost(enumeration = "HashOp", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub prehash_key: i32,
    #[prost(enumeration = "HashOp", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub prehash_value: i32,
    #[prost(enumeration = "LengthOp", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub length: i32,
    /// prefix is a fixed bytes that may optionally be included at the beginning to differentiate
    /// a leaf node from an inner node.
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// InnerOp represents a merkle-proof step that is not a leaf.
/// It represents concatenating two children and hashing them to provide the next result.
///
/// The result of the previous step is passed in, so the signature of this op is:
/// innerOp(child) -> output
///
/// The result of applying InnerOp should be:
/// output = op.hash(op.prefix || child || op.suffix)
///
/// where the || operator is concatenation of binary data,
/// and child is the result of hashing all the tree below this step.
///
/// Any special data, like prepending child with the length, or prepending the entire operation with
/// some value to differentiate from leaf nodes, should be included in prefix and suffix.
/// If either of prefix or suffix is empty, we just treat it as an empty string
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.InnerOp")]
pub struct InnerOp {
    #[prost(enumeration = "HashOp", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub hash: i32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub suffix: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// ProofSpec defines what the expected parameters are for a given proof type.
/// This can be stored in the client and used to validate any incoming proofs.
///
/// verify(ProofSpec, Proof) -> Proof | Error
///
/// As demonstrated in tests, if we don't fix the algorithm used to calculate the
/// LeafHash for a given tree, there are many possible key-value pairs that can
/// generate a given hash (by interpretting the preimage differently).
/// We need this for proper security, requires client knows a priori what
/// tree format server uses. But not in code, rather a configuration object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.ProofSpec")]
pub struct ProofSpec {
    /// any field in the ExistenceProof must be the same as in this spec.
    /// except Prefix, which is just the first bytes of prefix (spec can be longer)
    #[prost(message, optional, tag = "1")]
    pub leaf_spec: ::core::option::Option<LeafOp>,
    #[prost(message, optional, tag = "2")]
    pub inner_spec: ::core::option::Option<InnerSpec>,
    /// max_depth (if > 0) is the maximum number of InnerOps allowed (mainly for fixed-depth tries)
    #[prost(int32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_depth: i32,
    /// min_depth (if > 0) is the minimum number of InnerOps allowed (mainly for fixed-depth tries)
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_depth: i32,
    /// prehash_key_before_comparison is a flag that indicates whether to use the
    /// prehash_key specified by LeafOp to compare lexical ordering of keys for
    /// non-existence proofs.
    #[prost(bool, tag = "5")]
    pub prehash_key_before_comparison: bool,
}
///
/// InnerSpec contains all store-specific structure info to determine if two proofs from a
/// given store are neighbors.
///
/// This enables:
///
/// isLeftMost(spec: InnerSpec, op: InnerOp)
/// isRightMost(spec: InnerSpec, op: InnerOp)
/// isLeftNeighbor(spec: InnerSpec, left: InnerOp, right: InnerOp)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.InnerSpec")]
pub struct InnerSpec {
    /// Child order is the ordering of the children node, must count from 0
    /// iavl tree is [0, 1] (left then right)
    /// merk is [0, 2, 1] (left, right, here)
    #[prost(int32, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub child_order: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub child_size: i32,
    #[prost(int32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_prefix_length: i32,
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_prefix_length: i32,
    /// empty child is the prehash image that is used when one child is nil (eg. 20 bytes of 0)
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub empty_child: ::prost::alloc::vec::Vec<u8>,
    /// hash is the algorithm that must be used for each InnerOp
    #[prost(enumeration = "HashOp", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub hash: i32,
}
///
/// BatchProof is a group of multiple proof types than can be compressed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.BatchProof")]
pub struct BatchProof {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<BatchEntry>,
}
/// Use BatchEntry not CommitmentProof, to avoid recursion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.BatchEntry")]
pub struct BatchEntry {
    #[prost(oneof = "batch_entry::Proof", tags = "1, 2")]
    pub proof: ::core::option::Option<batch_entry::Proof>,
}
/// Nested message and enum types in `BatchEntry`.
pub mod batch_entry {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Proof {
        #[prost(message, tag = "1")]
        Exist(super::ExistenceProof),
        #[prost(message, tag = "2")]
        Nonexist(super::NonExistenceProof),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.CompressedBatchProof")]
pub struct CompressedBatchProof {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<CompressedBatchEntry>,
    #[prost(message, repeated, tag = "2")]
    pub lookup_inners: ::prost::alloc::vec::Vec<InnerOp>,
}
/// Use BatchEntry not CommitmentProof, to avoid recursion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.CompressedBatchEntry")]
pub struct CompressedBatchEntry {
    #[prost(oneof = "compressed_batch_entry::Proof", tags = "1, 2")]
    pub proof: ::core::option::Option<compressed_batch_entry::Proof>,
}
/// Nested message and enum types in `CompressedBatchEntry`.
pub mod compressed_batch_entry {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Proof {
        #[prost(message, tag = "1")]
        Exist(super::CompressedExistenceProof),
        #[prost(message, tag = "2")]
        Nonexist(super::CompressedNonExistenceProof),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.CompressedExistenceProof")]
pub struct CompressedExistenceProof {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub leaf: ::core::option::Option<LeafOp>,
    /// these are indexes into the lookup_inners table in CompressedBatchProof
    #[prost(int32, repeated, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub path: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.ics23.v1.CompressedNonExistenceProof")]
pub struct CompressedNonExistenceProof {
    /// TODO: remove this as unnecessary??? we prove a range
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub left: ::core::option::Option<CompressedExistenceProof>,
    #[prost(message, optional, tag = "3")]
    pub right: ::core::option::Option<CompressedExistenceProof>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum HashOp {
    /// NO_HASH is the default if no data passed. Note this is an illegal argument some places.
    NoHash = 0,
    Sha256 = 1,
    Sha512 = 2,
    Keccak = 3,
    Ripemd160 = 4,
    /// ripemd160(sha256(x))
    Bitcoin = 5,
    Sha512256 = 6,
}
impl HashOp {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HashOp::NoHash => "NO_HASH",
            HashOp::Sha256 => "SHA256",
            HashOp::Sha512 => "SHA512",
            HashOp::Keccak => "KECCAK",
            HashOp::Ripemd160 => "RIPEMD160",
            HashOp::Bitcoin => "BITCOIN",
            HashOp::Sha512256 => "SHA512_256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_HASH" => Some(Self::NoHash),
            "SHA256" => Some(Self::Sha256),
            "SHA512" => Some(Self::Sha512),
            "KECCAK" => Some(Self::Keccak),
            "RIPEMD160" => Some(Self::Ripemd160),
            "BITCOIN" => Some(Self::Bitcoin),
            "SHA512_256" => Some(Self::Sha512256),
            _ => None,
        }
    }
}
/// *
/// LengthOp defines how to process the key and value of the LeafOp
/// to include length information. After encoding the length with the given
/// algorithm, the length will be prepended to the key and value bytes.
/// (Each one with it's own encoded length)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum LengthOp {
    /// NO_PREFIX don't include any length info
    NoPrefix = 0,
    /// VAR_PROTO uses protobuf (and go-amino) varint encoding of the length
    VarProto = 1,
    /// VAR_RLP uses rlp int encoding of the length
    VarRlp = 2,
    /// FIXED32_BIG uses big-endian encoding of the length as a 32 bit integer
    Fixed32Big = 3,
    /// FIXED32_LITTLE uses little-endian encoding of the length as a 32 bit integer
    Fixed32Little = 4,
    /// FIXED64_BIG uses big-endian encoding of the length as a 64 bit integer
    Fixed64Big = 5,
    /// FIXED64_LITTLE uses little-endian encoding of the length as a 64 bit integer
    Fixed64Little = 6,
    /// REQUIRE_32_BYTES is like NONE, but will fail if the input is not exactly 32 bytes (sha256 output)
    Require32Bytes = 7,
    /// REQUIRE_64_BYTES is like NONE, but will fail if the input is not exactly 64 bytes (sha512 output)
    Require64Bytes = 8,
}
impl LengthOp {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LengthOp::NoPrefix => "NO_PREFIX",
            LengthOp::VarProto => "VAR_PROTO",
            LengthOp::VarRlp => "VAR_RLP",
            LengthOp::Fixed32Big => "FIXED32_BIG",
            LengthOp::Fixed32Little => "FIXED32_LITTLE",
            LengthOp::Fixed64Big => "FIXED64_BIG",
            LengthOp::Fixed64Little => "FIXED64_LITTLE",
            LengthOp::Require32Bytes => "REQUIRE_32_BYTES",
            LengthOp::Require64Bytes => "REQUIRE_64_BYTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_PREFIX" => Some(Self::NoPrefix),
            "VAR_PROTO" => Some(Self::VarProto),
            "VAR_RLP" => Some(Self::VarRlp),
            "FIXED32_BIG" => Some(Self::Fixed32Big),
            "FIXED32_LITTLE" => Some(Self::Fixed32Little),
            "FIXED64_BIG" => Some(Self::Fixed64Big),
            "FIXED64_LITTLE" => Some(Self::Fixed64Little),
            "REQUIRE_32_BYTES" => Some(Self::Require32Bytes),
            "REQUIRE_64_BYTES" => Some(Self::Require64Bytes),
            _ => None,
        }
    }
}
