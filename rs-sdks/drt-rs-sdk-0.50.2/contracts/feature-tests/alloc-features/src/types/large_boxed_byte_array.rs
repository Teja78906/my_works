use dharitri_sc::types::Box;
dharitri_sc::derive_imports!();

const ARRAY_SIZE: usize = 512;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LargeBoxedByteArray(Box<[u8; ARRAY_SIZE]>);
