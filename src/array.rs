//! Il2CppArray

use crate::class::Il2CppClass;
use crate::{NonNullRef, Ref};
use il2cpp_sys_rs::{
    il2cpp_array_class_get, il2cpp_array_get_byte_length, il2cpp_array_new,
    il2cpp_array_new_full, il2cpp_array_new_specific, il2cpp_array_size_t, il2cpp_bounded_array_class_get,
    il2cpp_class_array_element_size, il2cpp_gc_wbarrier_set_field, Il2CppArrayBounds,
    Il2CppTypeEnum_IL2CPP_TYPE_ARRAY, Il2CppTypeEnum_IL2CPP_TYPE_SZARRAY,
};
use std::any::type_name;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::{array, fmt, slice};

/// Array handle
pub type Il2CppArray<T, R> = NonNullRef<il2cpp_sys_rs::Il2CppArray, PhantomData<(T, R)>>;
/// Nullable Array handle
pub type Il2CppArrayRef<T, R> = Ref<il2cpp_sys_rs::Il2CppArray, PhantomData<(T, R)>>;

/// SZ Array handle
pub type Il2CppSzArray<T> = Il2CppArray<T, ()>;
/// Nullable SZ Array handle
pub type Il2CppSzArrayRef<T> = Il2CppArrayRef<T, ()>;

/// Multidimensional Array handle of rank `R`
///
/// IL2CPP supports ranks from 1 to 255 inclusive
pub type Il2CppMdArray<T, const R: usize> = Il2CppArray<T, Rank<R>>;
/// Nullable multidimensional Array handle of rank `R`
///
/// IL2CPP supports ranks from 1 to 255 inclusive
pub type Il2CppMdArrayRef<T, const R: usize> = Il2CppArrayRef<T, Rank<R>>;

/// IL2CPP array rank
///
/// IL2CPP supports ranks from 1 to 255 inclusive
#[derive(Clone, Copy)]
pub struct Rank<const R: usize>;

impl<T, R> Il2CppArray<T, R> {
    /// Return total number of elements
    ///
    /// # Note
    ///
    /// For multidimensional arrays, this is the product of all dimension lengths
    #[inline]
    pub const fn len(self) -> usize {
        self.as_ref().max_length
    }

    /// Total byte length of all array elements
    ///
    /// This equals to [`Self::element_size`] * [`Self::len`]
    #[inline]
    pub fn byte_len(self) -> usize {
        unsafe { il2cpp_array_get_byte_length(self.as_ptr()) as usize }
    }

    /// Pointer to the first element
    #[inline]
    pub const fn data_ptr(self) -> *mut T {
        // Safety: by IL2CPP layout, `T[0]` starts right after `Il2CppArray`.
        unsafe { self.as_ptr().add(1) as _ }
    }

    /// Size in bytes of one array element
    #[inline]
    pub const fn element_size(self) -> usize {
        Self::array_element_size(self.class())
    }

    /// Returns the runtime class of the array
    #[inline]
    pub const fn class(&self) -> Il2CppClass {
        unsafe { Il2CppClass::from_ptr(self.as_ref().obj.__bindgen_anon_1.klass) }.unwrap()
    }

    /// Returns the number of dimensions of the array
    #[inline]
    pub const fn rank(&self) -> u8 {
        self.class().as_ref().rank
    }

    /// Return true for single-dimensional, zero-based arrays
    #[inline]
    pub const fn is_sz(self) -> bool {
        self.as_ref().bounds.is_null()
    }

    /// Attempts to downcast this array to a single-dimensional, zero-based array
    ///
    /// # Returns
    ///
    /// `Some`([`Il2CppSzArray<T>`]) for SZ arrays.
    /// `None` for multidimensional or non-zero-based arrays
    #[inline]
    pub const fn try_as_sz(self) -> Option<Il2CppSzArray<T>> {
        if self.is_sz() {
            Some(Il2CppSzArray {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }
}

impl<T, R> Il2CppArray<T, R> {
    /// Size in bytes of one array element given the `array_class`
    ///
    /// # Arguments
    ///
    /// * `array_class` - Inflated array class
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    pub const fn array_element_size(array_class: Il2CppClass) -> usize {
        array_class.as_ref().element_size as usize
    }

    /// Size in bytes of one element of `class`
    ///
    /// # Arguments
    ///
    /// * `array_class` - Inflated class
    ///
    /// # Example
    ///
    /// Char types are of size `u16`\
    /// Reference types are of size `usize`
    #[inline]
    pub fn class_array_element_size(class: Il2CppClass) -> usize {
        unsafe { il2cpp_class_array_element_size(class.as_ptr()) as usize }
    }
}

impl<T> Il2CppSzArray<T> {
    /// Allocate a new SZ array
    ///
    /// # Performance
    ///
    /// For multiple allocations use [`Self::new_specific`]
    ///
    /// # Arguments
    ///
    /// * `element_class` - Inflated element class
    /// * `len` - Array length
    #[inline]
    pub fn new(element_class: Il2CppClass, len: il2cpp_array_size_t) -> Option<Self> {
        // Note: `il2cpp_array_new` use `il2cpp_array_class_get` then `il2cpp_array_new_specific`
        unsafe { Self::from_ptr(il2cpp_array_new(element_class.as_ptr(), len)) }
    }

    /// Allocate a new SZ array from an existing array class
    ///
    /// # Performance
    ///
    /// Create an array class using [`Self::array_class_get`]
    /// and reuse it for multiple allocations
    ///
    /// # Panics
    ///
    /// Panics if `array_class` is not a `IL2CPP_TYPE_SZARRAY`\
    /// Panics when `array_class::rank` != 1
    ///
    /// # Arguments
    ///
    /// * `array_class` - Inflated SZ array class
    /// * `len` - Array length
    #[track_caller]
    #[inline]
    pub fn new_specific(array_class: Il2CppClass, len: il2cpp_array_size_t) -> Option<Self> {
        assert_eq!(
            array_class.as_ref().byval_arg.type_(),
            Il2CppTypeEnum_IL2CPP_TYPE_SZARRAY
        );
        assert_eq!(array_class.as_ref().rank, 1);

        // Note: `il2cpp_array_new_specific` create only SZ array
        unsafe { Self::from_ptr(il2cpp_array_new_specific(array_class.as_ptr(), len)) }
    }

    /// Get the SZ array class for an element class
    ///
    /// Create the corresponding array using [`Self::new_specific`]
    ///
    /// # Arguments
    ///
    /// * `element_class` - Inflated element class
    #[inline]
    pub fn array_class_get(element_class: Il2CppClass) -> Option<Il2CppClass> {
        unsafe { Il2CppClass::from_ptr(il2cpp_array_class_get(element_class.as_ptr(), 1)) }
    }

    /// View as slice
    ///
    /// # Panics
    ///
    /// Panics if the array is not SZ
    #[track_caller]
    #[inline]
    pub const fn as_slice<'a>(self) -> &'a [T] {
        assert!(self.is_sz(), "array is not SZ");

        unsafe { slice::from_raw_parts(self.data_ptr(), self.len()) }
    }

    /// View as mutable slice
    ///
    /// # Safety
    ///
    /// Modifying elements must preserve data integrity.
    ///
    /// Reference types and boxed values require **special handling** to ensure
    /// they are correctly integrated into the managed memory pool.
    /// See [`Il2CppSzArray<_>::set_object`].
    ///
    /// # Panics
    ///
    /// Panics if the array is not SZ
    #[track_caller]
    #[inline]
    #[allow(unsafe_op_in_unsafe_fn)]
    pub const unsafe fn as_mut_slice<'a>(self) -> &'a mut [T] {
        assert!(self.is_sz(), "array is not SZ");

        slice::from_raw_parts_mut(self.data_ptr(), self.len())
    }

    /// Sets element at `index` to `value` for value type only.
    ///
    /// # Warning
    ///
    /// Use only if `T` is an owned value type. Otherwise, use [`Self::set_object`]
    ///
    /// # Panics
    ///
    /// Panics if the array is not SZ\
    /// Panics when `index > len`
    ///
    /// # Arguments
    ///
    /// * `index` - Index to write
    /// * `value` - Value to store
    #[track_caller]
    #[inline]
    pub const fn set_value(self, index: usize, value: T)
    where
        T: Copy + Send,
    {
        assert!(self.is_sz(), "array is not SZ");
        assert!(index < self.len(), "index > len");

        unsafe {
            self.as_mut_slice()[index] = value;
        }
    }
}

impl<T> Il2CppSzArray<*mut T> {
    /// Sets element at `index` to `value` using the IL2CPP GC write barrier
    ///
    /// # Warning
    ///
    /// Use only if `T` is a IL2CPP managed object. Otherwise, use [`Self::set_value`].
    ///
    /// # Safety
    ///
    /// `value` must be a managed object reference on the IL2CPP runtime
    ///
    /// # Panics
    ///
    /// Panics if the array is not SZ\
    /// Panics when `index > len`
    ///
    /// # Arguments
    ///
    /// * `index` - Index to write
    /// * `value` - Managed object pointer to store
    #[track_caller]
    #[inline]
    pub unsafe fn set_object(self, index: usize, value: *mut T) {
        assert!(self.is_sz(), "array is not SZ");
        assert!(index < self.len(), "index > len");

        unsafe {
            il2cpp_gc_wbarrier_set_field(
                self.as_ptr() as _,
                &mut self.as_mut_slice()[index] as *mut _ as _,
                value as *mut _ as _,
            )
        }
    }
}

impl<T, const R: usize> Il2CppMdArray<T, R> {
    /// Allocate a new MD array from an existing array class
    ///
    /// Use [`Self::bounded_array_class_get`] to create the specific `array_class`
    ///
    /// # Panics
    ///
    /// Panics if `array_class` is not a `IL2CPP_TYPE_ARRAY`\
    /// Panics when `array_class::rank` != `R`\
    /// Panics when `lower_bounds == [0]` for rank 1, as IL2CPP will create an SZ array
    ///
    /// # Arguments
    ///
    /// * `array_class` - Inflated MD array class of rank `R`
    /// * `lengths` - Per-dimension lengths
    /// * `lower_bounds` - Per-dimension lower bounds
    #[track_caller]
    #[inline]
    pub fn new(
        array_class: Il2CppClass,
        lengths: &mut [il2cpp_array_size_t; R],
        lower_bounds: &mut [il2cpp_array_size_t; R],
    ) -> Option<Self> {
        assert_eq!(
            array_class.as_ref().byval_arg.type_(),
            Il2CppTypeEnum_IL2CPP_TYPE_ARRAY
        );
        assert_eq!(array_class.as_ref().rank as usize, R);
        assert!(R > 1 || lower_bounds[0] != 0);

        unsafe {
            Self::from_ptr(il2cpp_array_new_full(
                array_class.as_ptr(),
                lengths.as_mut_ptr(),
                lower_bounds.as_mut_ptr(),
            ))
        }
    }

    /// Get the MD array class for an element class
    ///
    /// # Panics
    ///
    /// Panics when `R` == `0`
    ///
    /// # Arguments
    ///
    /// * `element_class` - Inflated element class
    #[track_caller]
    #[inline]
    pub fn bounded_array_class_get(element_class: Il2CppClass) -> Option<Il2CppClass> {
        assert_ne!(R, 0);

        unsafe {
            Il2CppClass::from_ptr(il2cpp_bounded_array_class_get(
                element_class.as_ptr(),
                R as u32,
                true,
            ))
        }
    }

    /// Array bounds
    ///
    /// # Panics
    ///
    /// Panics when `rank` != `R`
    #[track_caller]
    #[inline]
    pub fn bounds(self) -> [NonNull<Il2CppArrayBounds>; R] {
        assert_eq!(self.rank() as usize, R);

        let bounds =
            NonNull::new(self.as_ref().bounds).expect("MD array must have `bounds` defined");

        // Safety: bounds buffer is guaranteed to be the length of `rank`
        array::from_fn(|i| unsafe { bounds.add(i) })
    }
}

impl<T> Deref for Il2CppSzArray<T> {
    type Target = [T];

    #[track_caller]
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: fmt::Debug, const R: usize> fmt::Debug for Il2CppMdArray<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(array) = self.try_as_sz() {
            fmt::Debug::fmt(array.as_slice(), f)
        } else {
            write!(f, "Il2CppMdArray<{}, R={}>", type_name::<T>(), R)
        }
    }
}
