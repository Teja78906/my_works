use numbat_codec::{
    top_encode_number_to_output, EncodeError, EncodeErrorHandler, TopEncodeOutput, TryStaticCast,
};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

impl<M: ManagedTypeApi> TopEncodeOutput for &mut ManagedBuffer<M> {
    type NestedBuffer = ManagedBuffer<M>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.overwrite(bytes);
    }

    fn set_u64(self, value: u64) {
        // it could alternatively be performed via big int conversion
        // the most important thing is that no heap allocation happens
        self.overwrite(&[]);
        top_encode_number_to_output(self, value, false);
    }

    fn set_i64(self, value: i64) {
        // it could alternatively be performed via big int conversion
        // the most important thing is that no heap allocation happens
        self.overwrite(&[]);
        top_encode_number_to_output(self, value as u64, true);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>() || T::type_eq::<BigUint<M>>() || T::type_eq::<BigInt<M>>()
    }

    #[inline]
    fn set_specialized<T, H>(self, value: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            *self = managed_buffer.clone();
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<M>>() {
            *self = big_uint.to_bytes_be_buffer();
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<M>>() {
            *self = big_int.to_signed_bytes_be_buffer();
            Ok(())
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        (*self).clone()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        *self = nb;
    }
}
