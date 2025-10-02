use crate::{NonNullRef, Ref};
use il2cpp_sys_rs::Il2CppChar;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::string::FromUtf16Error;

/// Non-null managed string handle
pub type Il2CppString = NonNullRef<il2cpp_sys_rs::Il2CppString, ()>;
/// Nullable managed string handle
pub type Il2CppStringRef = Ref<il2cpp_sys_rs::Il2CppString, ()>;

impl Il2CppString {
    /// Return the string length in UTF-16 code units
    #[inline]
    pub const fn len(self) -> usize {
        unsafe { self.ptr.as_ref() }.length as usize
    }

    /// Return true if the string is empty
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Return pointer to UTF-16 characters
    #[inline]
    pub const fn chars_ptr(self) -> *const Il2CppChar {
        unsafe { self.ptr.as_ref() }.chars.as_ptr()
    }

    /// Return mutable pointer to UTF-16 characters
    #[inline]
    pub const fn chars_ptr_mut(mut self) -> *mut Il2CppChar {
        unsafe { self.ptr.as_mut() }.chars.as_mut_ptr()
    }

    /// View contents as UTF-16 slice
    #[inline]
    pub const fn as_slice<'a>(self) -> &'a [Il2CppChar] {
        unsafe { std::slice::from_raw_parts(self.chars_ptr(), self.len()) }
    }

    /// View contents as mutable UTF-16 slice
    #[inline]
    pub const fn as_mut_slice<'a>(self) -> &'a mut [Il2CppChar] {
        unsafe { std::slice::from_raw_parts_mut(self.chars_ptr_mut(), self.len()) }
    }

    /// Convert from UTF-16 code units to a Rust `String` (UTF-8).
    ///
    /// # Errors
    ///
    /// If the UTF-16 data is invalid.
    pub fn to_utf16(self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.as_slice())
    }

    /// Convert from UTF-16 code units to a Rust `String` (UTF-8),
    /// replacing invalid sequences with [the replacement character (`U+FFFD`)][U+FFFD].
    pub fn to_utf16_lossy(self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

impl PartialEq for Il2CppString {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl Eq for Il2CppString {}

impl PartialOrd for Il2CppString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Il2CppString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Hash for Il2CppString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl Deref for Il2CppString {
    type Target = [Il2CppChar];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Il2CppString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl fmt::Display for Il2CppString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_utf16_lossy(), f)
    }
}

impl fmt::Debug for Il2CppString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_utf16_lossy(), f)
    }
}

impl fmt::Debug for Il2CppStringRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.non_null())
    }
}
