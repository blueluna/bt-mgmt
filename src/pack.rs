//! # Traits for handling packing and unpacking
//!
//! These traits handles packing and unpacking of data into byte slices

/// Packing of data of fixed size
pub trait PackFixed<T, E> {
    /// Serialise into buffer, returning if there was an error
    fn pack(&self, data: &mut [u8]) -> Result<(), E>;
}

/// Unpacking of data of fixed size
pub trait UnpackFixed<'a, T, E> {
    /// De-serialise from buffer, returning object or error
    fn unpack(data: &'a [u8]) -> Result<T, E>;
}

/// Packing of data with variable size
pub trait Pack<T, E> {
    /// Serialise into buffer, returning number of bytes written or error
    fn pack(&self, data: &mut [u8]) -> Result<usize, E>;
}

/// Unpacking of data with variable size
pub trait Unpack<'a, T, E> {
    /// De-serialise from buffer, returning object and number of bytes used
    /// or error
    fn unpack(data: &'a [u8]) -> Result<(T, usize), E>;
}
