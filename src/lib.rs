extern crate core;

mod array;
mod class;
mod string;

pub mod sys {
    pub use il2cpp_sys_rs::*;
}

pub use array::*;
pub use class::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
pub use string::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Ref<T, G> {
    ptr: *mut T,
    _marker: PhantomData<G>,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct NonNullRef<T, G> {
    ptr: ::core::ptr::NonNull<T>,
    _marker: PhantomData<G>,
}

impl<T, G> Ref<T, G> {
    #[inline]
    pub const fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.ptr
    }

    #[inline]
    pub const fn is_null(self) -> bool {
        self.ptr.is_null()
    }

    #[inline]
    pub const fn as_ref<'a>(self) -> Option<&'a T> {
        unsafe { self.ptr.as_ref() }
    }

    #[inline]
    pub const fn as_mut<'a>(self) -> Option<&'a mut T> {
        unsafe { self.ptr.as_mut() }
    }

    #[inline]
    pub const fn expect<'a>(self, msg: &str) -> &'a T {
        self.as_ref().expect(msg)
    }

    #[inline]
    pub const fn expect_mut<'a>(self, msg: &str) -> &'a mut T {
        self.as_mut().expect(msg)
    }

    #[inline]
    pub const fn non_null(self) -> Option<NonNullRef<T, G>> {
        if let Some(ptr) = ::core::ptr::NonNull::new(self.ptr) {
            Some(NonNullRef {
                ptr,
                _marker: self._marker,
            })
        } else {
            None
        }
    }

    #[inline]
    pub const fn unwrap_non_null(self) -> NonNullRef<T, G> {
        self.non_null().unwrap()
    }
}

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

impl<T, G> NonNullRef<T, G> {
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[inline]
    pub const fn as_ref<'a>(self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }

    #[inline]
    pub const fn as_mut<'a>(mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}
