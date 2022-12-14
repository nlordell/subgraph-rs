//! Module containing cryptographic utility functions.

use crate::ffi::{buf::AscTypedArray, sys};

/// A 32-byte hash.
pub type Hash = [u8; 32];

/// Computes the Keccak-256 hash of the specified input bytes.
pub fn keccak256(data: impl AsRef<[u8]>) -> Hash {
    let data = data.as_ref();
    let array = AscTypedArray::from_bytes(data);
    let digest = unsafe { &*sys::crypto__keccak256(array.as_ptr()) };
    digest.as_slice().try_into().unwrap()
}
