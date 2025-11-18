//! Safe Rust bindings for IL2CPP internals

mod array;
mod assembly;
mod class;
mod constants;
mod exception;
mod field_info;
mod image;
mod method_info;
mod property_info;
mod string;

/// Raw IL2CPP bindings
pub mod sys {
    pub use il2cpp_sys_rs::*;
}

pub use array::*;
pub use assembly::*;
pub use class::*;
pub use exception::*;
pub use field_info::*;
pub use image::*;
pub use method_info::*;
pub use property_info::*;
use std::any::type_name;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{fmt, ptr};
pub use string::*;

/// Nullable managed IL2CPP reference
#[repr(transparent)]
pub struct Ref<T, G> {
    ptr: *mut T,
    _marker: PhantomData<G>,
}

/// Non-null managed IL2CPP reference
#[repr(transparent)]
pub struct NonNullRef<T, G> {
    ptr: NonNull<T>,
    _marker: PhantomData<G>,
}

impl<T, G> Ref<T, G> {
    /// Creates a new reference from a raw pointer
    ///
    /// # Arguments
    ///
    /// * `ptr` - Raw pointer
    #[inline]
    pub const fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a null reference
    #[inline]
    pub const fn null() -> Self {
        Self {
            ptr: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    /// Returns the inner raw pointer
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.ptr
    }

    /// Whether the reference is null
    #[inline]
    pub const fn is_null(self) -> bool {
        self.ptr.is_null()
    }

    /// Immutable reference to the value if not null
    #[inline]
    pub const fn as_ref<'a>(self) -> Option<&'a T> {
        unsafe { self.ptr.as_ref() }
    }

    /// Mutable reference to the value if not null
    #[inline]
    pub const fn as_mut<'a>(self) -> Option<&'a mut T> {
        unsafe { self.ptr.as_mut() }
    }

    /// Immutable reference
    ///
    /// # Panics
    ///
    /// Panics if the value is null
    ///
    /// # Arguments
    ///
    /// * `msg` - Panic message
    #[track_caller]
    #[inline]
    pub const fn expect<'a>(self, msg: &str) -> &'a T {
        self.as_ref().expect(msg)
    }

    /// Mutable reference
    ///
    /// # Arguments
    ///
    /// * `msg` - Panic message
    ///
    /// # Panics
    ///
    /// Panics if the value is null
    #[inline]
    pub const fn expect_mut<'a>(self, msg: &str) -> &'a mut T {
        self.as_mut().expect(msg)
    }

    /// Converts to a non-null reference if the value is not null
    #[inline]
    pub const fn non_null(self) -> Option<NonNullRef<T, G>> {
        if let Some(ptr) = NonNull::new(self.ptr) {
            Some(NonNullRef {
                ptr,
                _marker: self._marker,
            })
        } else {
            None
        }
    }

    /// Converts to a non-null reference
    ///
    /// # Panics
    ///
    /// Panics when called on a null reference
    #[track_caller]
    #[inline]
    pub const fn unwrap_non_null(self) -> NonNullRef<T, G> {
        self.non_null().unwrap()
    }
}

impl<T, G> Clone for Ref<T, G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, G> Copy for Ref<T, G> {}

impl<T, G> PartialEq for Ref<T, G> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr.eq(&other.ptr)
    }
}

impl<T, G> Eq for Ref<T, G> {}

impl<T, G> PartialOrd for Ref<T, G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, G> Ord for Ref<T, G> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ptr.cmp(&other.ptr)
    }
}

impl<T, G> Hash for Ref<T, G> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state)
    }
}

impl<T, G> From<*mut T> for Ref<T, G> {
    #[inline]
    fn from(value: *mut T) -> Self {
        Self::new(value)
    }
}

impl<T, G> From<NonNullRef<T, G>> for Ref<T, G> {
    #[inline]
    fn from(value: NonNullRef<T, G>) -> Self {
        Self::new(value.as_ptr())
    }
}

impl<T, G> Default for Ref<T, G> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T, G> fmt::Debug for Ref<T, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>()).field(&self.ptr).finish()
    }
}

impl<T, G> NonNullRef<T, G> {
    /// Creates a non-null reference from a raw pointer
    ///
    /// # Arguments
    ///
    /// * `ptr` - Raw pointer
    ///
    /// # Returns
    ///
    /// Non-null reference if `ptr` is not null, otherwise `None`
    #[inline]
    pub const fn from_ptr(ptr: *mut T) -> Option<Self> {
        if let Some(ptr) = NonNull::new(ptr) {
            Some(Self {
                ptr,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    /// Returns the raw non-null pointer
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Immutable reference to the value
    #[inline]
    pub const fn as_ref<'a>(self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }

    /// Mutable reference to the value
    #[inline]
    pub const fn as_mut<'a>(mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T, G> Clone for NonNullRef<T, G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, G> Copy for NonNullRef<T, G> {}
