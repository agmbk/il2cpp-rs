//! Il2CppString

use crate::{NonNullRef, Ref};
use il2cpp_sys_rs::{il2cpp_string_new, il2cpp_string_new_utf16, Il2CppChar};
use std::cmp::Ordering;
use std::ffi::CStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::string::FromUtf16Error;

/// String handle
pub type Il2CppString = NonNullRef<il2cpp_sys_rs::Il2CppString, ()>;
/// Nullable String handle
pub type Il2CppStringRef = Ref<il2cpp_sys_rs::Il2CppString, ()>;

impl Il2CppString {
    /// Allocate a new string
    ///
    /// # Arguments
    ///
    /// * `s` - Null-terminated C string
    ///
    /// # Returns
    ///
    /// [`Some(Il2CppString)`] if allocation succeeds, otherwise `None`.
    #[inline]
    pub fn new(s: &CStr) -> Option<Self> {
        Self::from_ptr(unsafe { il2cpp_string_new(s.as_ptr()) })
    }

    /// Allocate a new string from a UTF-16 null-terminated slice
    ///
    /// # Safety
    ///
    /// `s` must be null-terminated
    ///
    /// # Arguments
    ///
    /// * `s` - Null-terminated UTF-16 slice
    ///
    /// # Returns
    ///
    /// [`Some(Il2CppString)`] if allocation succeeds, otherwise `None`.
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    pub unsafe fn new_utf16(s: &[Il2CppChar], len: i32) -> Option<Self> {
        Self::from_ptr(il2cpp_string_new_utf16(s.as_ptr(), len))
    }

    /// Interns this string and returns the canonical interned instance.
    ///
    /// Uses the runtime's intern pool. Equal strings share the same instance.
    ///
    /// # Returns
    ///
    /// [`Some(Il2CppString)`] representing the interned string, or `None` if interning fails.
    #[inline]
    pub fn intern(self) -> Option<Self> {
        Self::from_ptr(unsafe { il2cpp_sys_rs::il2cpp_string_intern(self.as_ptr()) })
    }

    /// Returns the interned instance if one already exists.
    ///
    /// Does not modify the intern pool. Only queries it.
    #[inline]
    pub fn is_interned(self) -> Option<Self> {
        let ptr = unsafe { il2cpp_sys_rs::il2cpp_string_is_interned(self.as_ptr()) };
        Self::from_ptr(ptr)
    }

    /// Returns the string length in UTF-16 code units
    #[inline]
    pub const fn len(self) -> usize {
        self.as_ref().length as usize
    }

    /// Returns `true` if the string is empty
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns a pointer to the UTF-16 characters
    #[inline]
    pub const fn chars_ptr(self) -> *const Il2CppChar {
        self.as_ref().chars.as_ptr()
    }

    /// Returns a mutable pointer to the UTF-16 characters
    ///
    /// # Safety
    ///
    /// The caller must ensure the string contents remain valid UTF-16
    #[inline]
    pub const unsafe fn chars_ptr_mut(self) -> *mut Il2CppChar {
        self.as_mut().chars.as_mut_ptr()
    }

    /// View contents as UTF-16 slice
    #[inline]
    pub const fn as_slice<'a>(self) -> &'a [Il2CppChar] {
        unsafe { std::slice::from_raw_parts(self.chars_ptr(), self.len()) }
    }

    /// View contents as mutable UTF-16 slice
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice contents remain valid UTF-16
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    pub const unsafe fn as_mut_slice<'a>(self) -> &'a mut [Il2CppChar] {
        std::slice::from_raw_parts_mut(self.chars_ptr_mut(), self.len())
    }

    /// Convert from UTF-16 code units to a Rust UTF-8 [`String`]
    ///
    /// # Errors
    ///
    /// Returns `Err` if the UTF-16 data cannot be converted to valid UTF-8
    #[inline]
    pub fn to_utf16(self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.as_slice())
    }

    /// Convert from UTF-16 code units to a Rust UTF-8 [`String`],
    /// replacing invalid sequences with [the replacement character (`U+FFFD`)][U+FFFD].
    #[inline]
    pub fn to_utf16_lossy(self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

impl PartialEq for Il2CppString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl Eq for Il2CppString {}

impl PartialOrd for Il2CppString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Il2CppString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Hash for Il2CppString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl Deref for Il2CppString {
    type Target = [Il2CppChar];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
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
