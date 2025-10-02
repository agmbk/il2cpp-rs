use crate::class::Il2CppClass;
use crate::class::Il2CppClassRef;
use crate::{NonNullRef, Ref};
use il2cpp_sys_rs::{
    il2cpp_array_class_get, il2cpp_array_element_size, il2cpp_array_get_byte_length, il2cpp_array_length,
    il2cpp_array_new, il2cpp_array_new_full, il2cpp_array_size_t, il2cpp_bounded_array_class_get,
    il2cpp_class_array_element_size, Il2CppArray,
};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{fmt, slice};

#[derive(Clone, Copy)]
pub struct Rank<const R: usize>;

/// Non-null IL2CPP SZ array
pub type Il2CppSzArray<T> = Il2CppMdArray<T, 0>;
/// Nullable IL2CPP SZ array
pub type Il2CppSzArrayRef<T> = Il2CppMdArrayRef<T, 0>;

/// Non-null IL2CPP multidimensional array of rank `R`
pub type Il2CppMdArray<T, const R: usize> = NonNullRef<Il2CppArray, (PhantomData<T>, Rank<R>)>;
/// Nullable IL2CPP multidimensional array of rank `R`
pub type Il2CppMdArrayRef<T, const R: usize> = Ref<Il2CppArray, (PhantomData<T>, Rank<R>)>;

impl<T, const R: usize> Il2CppMdArray<T, R> {
    /// Allocate a new multidimensional array
    ///
    /// # Arguments
    ///
    /// * `array_class` - Array runtime class for rank [`R`]
    /// * `lengths` - lengths of each dimension
    /// * `lower_bounds` - lower bounds of each dimension
    ///
    /// # Safety
    ///
    /// `elem_class` must be a valid for the array rank [`R`]
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    pub unsafe fn new_md(
        elem_class: Il2CppClass,
        lengths: &mut [il2cpp_array_size_t; R],
        lower_bounds: &mut [il2cpp_array_size_t; R],
    ) -> Il2CppMdArrayRef<T, R> {
        Ref::from_ptr(il2cpp_array_new_full(
            elem_class.as_ptr(),
            lengths.as_mut_ptr(),
            lower_bounds.as_mut_ptr(),
        ))
    }

    /// Obtain (or create) the IL2CPP array class for an element class and rank.
    ///
    /// Runtime's canonical way to get the `System.Array` derived
    /// [`Il2CppClass`] for [`T[,]`] of a given `rank`.
    #[inline]
    pub fn array_class_get(element_class: Il2CppClass) -> Option<Il2CppClass> {
        unsafe {
            Ref::from_ptr(il2cpp_array_class_get(element_class.as_ptr(), R as u32 + 1)).non_null()
        }
    }

    /// Obtain the IL2CPP array class with/without bounds metadata.
    ///
    /// When `bounded = true`, IL2CPP returns an array class that supports
    /// non–zero-based bounds.
    /// For regular SZ arrays, use `bounded = false`.
    #[inline]
    pub fn bounded_array_class_get(element_class: Il2CppClass, bounded: bool) -> Il2CppClassRef {
        unsafe {
            Ref::from_ptr(il2cpp_bounded_array_class_get(
                element_class.as_ptr(),
                R as u32 + 1,
                bounded,
            ))
        }
    }

    /// Downcast to SZ array when the runtime shape is single-dimensional and zero-based
    ///
    /// # Returns
    ///
    /// Some([`Il2CppSzArray<T>`]) for SZ arrays.
    /// `None` for multidimensional or non-zero-based arrays.
    #[inline]
    pub const fn try_as_sz(self) -> Option<Il2CppSzArray<T>> {
        if self.is_szarray() {
            Some(Il2CppSzArray {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    /// Return true for single-dimensional, zero-based arrays
    #[inline]
    pub const fn is_szarray(self) -> bool {
        unsafe { self.ptr.as_ref() }.bounds.is_null()
    }

    /// Return total number of elements
    #[inline]
    pub fn len(self) -> usize {
        unsafe { il2cpp_array_length(self.ptr.as_ptr()) as usize }
    }

    /// Size in bytes of one array element given the **array class**.
    ///
    /// # Safety
    ///
    /// - `array_class` must be a valid `Il2CppClass*` for an array type.
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    pub unsafe fn array_element_size(array_class: Il2CppClass) -> usize {
        il2cpp_array_element_size(array_class.as_ptr()) as usize
    }

    /// Size in bytes of one element for **klass** when `klass` represents an array.
    /// If `klass` is not an array class, size will compute based on the element type it represents.
    ///
    /// Handles value types vs reference types.
    #[inline]
    pub fn class_array_element_size(klass: Il2CppClass) -> usize {
        unsafe { il2cpp_class_array_element_size(klass.as_ptr()) as usize }
    }

    /// Return number of elements in dimension `d`
    ///
    /// # Panics
    ///
    /// Panics if `d` >= `R`
    #[inline]
    pub const fn len_dim(self, d: usize) -> usize {
        assert!(d < R);
        unsafe {
            let array = self.ptr.as_ref();
            if R == 1 && array.bounds.is_null() {
                array.max_length
            } else {
                (*array.bounds.add(d)).length
            }
        }
    }

    /// Return lower bound for dimension `d`
    ///
    /// # Panics
    ///
    /// Panics if `d` >= `R`
    #[inline]
    pub const fn lb_dim(self, d: usize) -> isize {
        assert!(d < R);
        unsafe {
            let a = self.ptr.as_ref();
            if R == 1 && a.bounds.is_null() {
                0
            } else {
                (*a.bounds.add(d)).lower_bound as isize
            }
        }
    }

    /// Byte length
    #[inline]
    pub fn byte_len(self) -> usize {
        unsafe { il2cpp_array_get_byte_length(self.as_ptr()) as usize }
    }
}

impl<T> Il2CppSzArray<T> {
    /// Allocate a new SZ array
    ///
    /// # Arguments
    ///
    /// * `elem_class` - Element runtime class
    /// * `len` - Element count
    ///
    /// # Safety
    ///
    /// `elem_class` must be a valid pointer
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    pub unsafe fn new(elem_class: Il2CppClass, len: il2cpp_array_size_t) -> Option<Self> {
        Ref::from_ptr(il2cpp_array_new(elem_class.as_ptr(), len)).non_null()
    }

    // this is false since we do not know which rank is associated to the provided array_class.
    //
    // /// Allocate with array class resolved from element class
    // ///
    // /// # Arguments
    // ///
    // /// * `elem_class` - Element runtime class
    // /// * `len` - Element count
    // ///
    // /// # Safety
    // ///
    // /// `elem_class` must be a valid pointer
    // #[allow(unsafe_op_in_unsafe_fn)]
    // #[inline]
    // pub unsafe fn new_specific(array_class: Il2CppClass, len: il2cpp_array_size_t) -> Il2CppSzArrayRef<T> {
    //     Ref::from_ptr(il2cpp_array_new_specific(array_class.as_ptr(), len))
    // }

    /// Pointer to the first element
    #[inline]
    pub const fn data_ptr(self) -> *mut T {
        assert!(self.is_szarray(), "Non-SZ array");
        // Safety: by IL2CPP layout, `T[0]` starts right after `Il2CppArray`.
        unsafe { self.ptr.as_ptr().add(1) as _ }
    }

    /// Immutable element pointer at logical index `i`
    #[inline]
    pub const fn elem_ptr(self, i: usize) -> *const T {
        assert!(self.is_szarray(), "Non-SZ array");
        unsafe { self.data_ptr().add(i) }
    }

    /// Mutable element pointer at logical index `i`
    #[inline]
    pub const fn elem_ptr_mut(self, i: usize) -> *mut T {
        assert!(self.is_szarray(), "Non-SZ array");
        unsafe { self.data_ptr().add(i) }
    }

    /// View as slice
    ///
    /// # Panics
    ///
    /// Panics if the array is not single-dimensional and zero-based
    #[inline]
    pub fn as_slice<'a>(self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.data_ptr(), self.len()) }
    }

    /// View as mutable slice
    ///
    /// # Panics
    ///
    /// Panics if the array is not single-dimensional and zero-based
    #[inline]
    pub fn as_mut_slice<'a>(self) -> &'a mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data_ptr(), self.len()) }
    }
}

impl<T> Deref for Il2CppSzArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for Il2CppSzArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: fmt::Debug, const R: usize> fmt::Debug for Il2CppMdArray<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(array) = self.try_as_sz() {
            // Safety: array is guaranteed to be SZ
            fmt::Debug::fmt(array.as_slice(), f)
        } else {
            write!(f, "Il2CppMdArray<{}, R={}>", core::any::type_name::<T>(), R)
        }
    }
}

impl<T: fmt::Debug, const R: usize> fmt::Debug for Il2CppMdArrayRef<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.non_null())
    }
}
