//! Il2CppException

use crate::{NonNullRef, Ref};
use il2cpp_sys_rs::il2cpp_format_exception;
use std::ffi::{CStr, CString};
use std::fmt;

/// Exception handle
pub type Exception = NonNullRef<il2cpp_sys_rs::Il2CppException, ()>;
/// Nullable Exception handle
pub type ExceptionRef = Ref<il2cpp_sys_rs::Il2CppException, ()>;

impl Exception {
    /// Format the exception into a string
    ///
    /// If the formatted exception exceeds `N` bytes, the output will be truncated.
    ///
    /// # Type Parameters
    ///
    /// * `N` - Size of the buffer used for formatting
    pub fn format<const N: usize>(self) -> CString {
        unsafe {
            let mut buffer = vec![0i8; N];
            il2cpp_format_exception(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            CStr::from_ptr(buffer.as_ptr()).to_owned()
        }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.format::<1024>().to_string_lossy(), f)
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.format::<1024>().to_string_lossy(), f)
    }
}
