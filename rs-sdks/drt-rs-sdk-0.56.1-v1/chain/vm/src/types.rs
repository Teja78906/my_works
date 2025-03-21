pub use crate::chain_core::types::Address as VMAddress;
pub use crate::chain_core::types::CodeMetadata as VMCodeMetadata;
pub use crate::chain_core::types::DcdtLocalRole;
pub use crate::chain_core::types::DcdtLocalRoleFlags;
pub use crate::chain_core::types::TokenType as VMTokenType;
pub use crate::chain_core::types::H256;

pub type RawHandle = i32;

use num_bigint::BigUint;
use num_traits::Zero;

/// Helper function to quickly encode a u64 value, according to the DharitrI codec format.
pub fn top_encode_u64(value: u64) -> Vec<u8> {
    top_encode_big_uint(&BigUint::from(value))
}

/// Helper function to quickly encode a BigUint value, according to the DharitrI codec format.
pub fn top_encode_big_uint(value: &BigUint) -> Vec<u8> {
    if value.is_zero() {
        Vec::new()
    } else {
        value.to_bytes_be()
    }
}

pub(crate) fn top_decode_u64(bytes: &[u8]) -> u64 {
    BigUint::from_bytes_be(bytes).try_into().unwrap()
}
