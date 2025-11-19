//! FieldInfo

use crate::constants::{
    FIELD_ATTRIBUTE_FIELD_ACCESS_MASK, FIELD_ATTRIBUTE_INIT_ONLY, FIELD_ATTRIBUTE_LITERAL,
    FIELD_ATTRIBUTE_STATIC,
};
use crate::{Il2CppClass, NonNullRef, Ref};
use il2cpp_sys_rs::{
    il2cpp_class_get_field_from_name, il2cpp_field_get_value, il2cpp_field_static_get_value, il2cpp_type_get_name,
    Il2CppObject, Il2CppType,
};
use std::ffi::CStr;
use std::{fmt, mem};

/// FieldInfo handle
pub type FieldInfo = NonNullRef<il2cpp_sys_rs::FieldInfo, ()>;
/// Nullable FieldInfo handle
pub type FieldInfoRef = Ref<il2cpp_sys_rs::FieldInfo, ()>;

impl FieldInfo {
    /// Returns the field name
    #[inline]
    pub const fn name(self) -> &'static CStr {
        // Safety: `name` is never null
        unsafe { CStr::from_ptr(self.as_ref().name) }
    }

    /// Returns the field type
    ///
    /// # Panics
    ///
    /// Panics if the type pointer is null
    #[track_caller]
    #[inline]
    pub const fn type_(self) -> NonNullRef<Il2CppType, ()> {
        Ref::new(self.as_ref().type_ as _).unwrap_non_null()
    }

    /// Returns the parent class of the field
    ///
    /// # Panics
    ///
    /// Panics if the parent class pointer is null
    #[track_caller]
    #[inline]
    pub const fn parent(self) -> Il2CppClass {
        Il2CppClass::from_ptr(self.as_ref().parent as _).unwrap()
    }

    /// Returns the byte offset of the field within its containing class
    #[inline]
    pub const fn offset(self) -> usize {
        self.as_ref().offset as usize
    }

    /// Returns the raw field flags
    #[inline]
    pub fn flags(self) -> u32 {
        self.type_().as_ref().attrs()
    }

    /// Field accessibility
    ///
    /// - 0x0000 - `[CompilerGenerated]`
    /// - 0x0001 - `private`
    /// - 0x0002 - `private protected`
    /// - 0x0003 - `internal`
    /// - 0x0004 - `protected`
    /// - 0x0005 - `protected internal`
    /// - 0x0006 - `public`
    #[inline]
    pub fn accessibility(self) -> u32 {
        self.flags() & FIELD_ATTRIBUTE_FIELD_ACCESS_MASK
    }

    /// Returns `true` if field is static
    #[inline]
    pub fn is_static(&self) -> bool {
        self.flags() & FIELD_ATTRIBUTE_STATIC != 0
    }

    /// Returns `true` if field is read-only
    #[inline]
    pub fn is_readonly(&self) -> bool {
        self.flags() & FIELD_ATTRIBUTE_INIT_ONLY != 0
    }

    /// Returns `true` if the field is a compile-time constant
    #[inline]
    pub fn is_const(&self) -> bool {
        self.flags() & FIELD_ATTRIBUTE_LITERAL != 0
    }

    /// Get the field value
    ///
    /// # Safety
    ///
    /// The field must be of type `T`
    ///
    /// # Panics
    ///
    /// Panics if the field is static
    ///
    /// # Arguments
    ///
    /// * `this` - Class instance containing the field.
    #[allow(unsafe_op_in_unsafe_fn)]
    #[track_caller]
    #[inline]
    pub unsafe fn value<T>(self, this: *mut Il2CppObject) -> T {
        assert!(!self.is_static());

        let mut value = mem::zeroed();
        il2cpp_field_get_value(this as _, self.as_ptr(), &mut value as *mut _ as _);
        value
    }

    /// Get the static field value
    ///
    /// # Safety
    ///
    /// The field must be of type `T`
    ///
    /// # Panics
    ///
    /// Panics if the field is not static
    #[allow(unsafe_op_in_unsafe_fn)]
    #[track_caller]
    #[inline]
    pub unsafe fn static_value<T>(self) -> T {
        assert!(self.is_static());

        let mut value = mem::zeroed();
        il2cpp_field_static_get_value(self.as_ptr(), &mut value as *mut _ as _);
        value
    }

    /// Returns the field token
    #[inline]
    pub const fn token(self) -> u32 {
        self.as_ref().token
    }
}

impl FieldInfo {
    /// Finds a field by name
    ///
    /// # Arguments
    ///
    /// * `class` - Class containing the field
    /// * `name` - Field name
    ///
    /// # Returns
    ///
    /// FieldInfo handle if found, otherwise `None`
    #[inline]
    pub(crate) fn from_name(class: Il2CppClass, name: &CStr) -> Option<Self> {
        unsafe {
            Ref::new(il2cpp_class_get_field_from_name(
                class.as_ptr(),
                name.as_ptr(),
            ))
            .non_null()
        }
    }
}

impl fmt::Display for FieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl fmt::Debug for FieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldInfo")
            .field("name", &self.name().to_string_lossy())
            .field("type", &unsafe {
                CStr::from_ptr(il2cpp_type_get_name(self.type_().as_ptr())).to_string_lossy()
            })
            .field("offset", &self.offset())
            .finish()
    }
}
