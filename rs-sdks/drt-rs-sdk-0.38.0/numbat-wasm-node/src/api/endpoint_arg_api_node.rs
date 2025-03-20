use crate::VmApiImpl;
use numbat_wasm::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl},
    types::heap::BoxedBytes,
};

extern "C" {
    fn getNumArguments() -> i32;
    fn getArgumentLength(id: i32) -> i32;
    fn getArgument(id: i32, dstOffset: *mut u8) -> i32;

    // managed buffer API (currently the main one)
    fn mBufferGetArgument(argId: i32, mBufferHandle: i32) -> i32;

    // big int API
    fn bigIntGetUnsignedArgument(arg_index: i32, dest: i32);
    fn bigIntGetSignedArgument(arg_index: i32, dest: i32);

    // small int API
    fn smallIntGetUnsignedArgument(id: i32) -> i64;
    fn smallIntGetSignedArgument(id: i32) -> i64;

    // callback closure directly from the VM
    fn managedGetCallbackClosure(callbackClosureHandle: i32);
}

impl EndpointArgumentApi for VmApiImpl {
    type EndpointArgumentApiImpl = VmApiImpl;

    #[inline]
    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        VmApiImpl {}
    }
}

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl EndpointArgumentApiImpl for VmApiImpl {
    #[inline]
    fn get_num_arguments(&self) -> i32 {
        unsafe { getNumArguments() }
    }

    #[inline]
    fn get_argument_len(&self, arg_index: i32) -> usize {
        unsafe { getArgumentLength(arg_index) as usize }
    }

    #[inline]
    fn load_argument_managed_buffer(&self, arg_index: i32, dest: Self::ManagedBufferHandle) {
        unsafe {
            mBufferGetArgument(arg_index, dest);
        }
    }

    fn get_argument_boxed_bytes(&self, arg_index: i32) -> BoxedBytes {
        let len = self.get_argument_len(arg_index);
        unsafe {
            let mut res = BoxedBytes::allocate(len);
            if len > 0 {
                getArgument(arg_index, res.as_mut_ptr());
            }
            res
        }
    }

    #[inline]
    fn load_argument_big_int_unsigned(&self, arg_index: i32, dest: Self::ManagedBufferHandle) {
        unsafe {
            bigIntGetUnsignedArgument(arg_index, dest);
        }
    }

    #[inline]
    fn load_argument_big_int_signed(&self, arg_index: i32, dest: Self::ManagedBufferHandle) {
        unsafe {
            bigIntGetSignedArgument(arg_index, dest);
        }
    }

    #[inline]
    fn get_argument_u64(&self, arg_index: i32) -> u64 {
        unsafe { smallIntGetUnsignedArgument(arg_index) as u64 }
    }

    #[inline]
    fn get_argument_i64(&self, arg_index: i32) -> i64 {
        unsafe { smallIntGetSignedArgument(arg_index) }
    }

    #[inline]
    fn load_callback_closure_buffer(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetCallbackClosure(dest);
        }
    }
}
