//! Il2CppClass

use crate::{FieldInfo, Il2CppImage, MethodInfo, NonNullRef, PropertyInfo, Ref};
use il2cpp_sys_rs::{
    il2cpp_class_array_element_size, il2cpp_class_from_name, il2cpp_class_get_type, il2cpp_class_is_abstract,
    il2cpp_class_is_blittable, il2cpp_class_is_enum, il2cpp_class_is_generic,
    il2cpp_class_is_inflated, il2cpp_class_is_inited, il2cpp_class_is_interface,
    il2cpp_class_is_subclass_of, il2cpp_class_is_valuetype, il2cpp_class_num_fields,
    Il2CppType,
};
use std::borrow::Cow;
use std::ffi::CStr;
use std::{fmt, slice};

/// Class handle
pub type Il2CppClass = NonNullRef<il2cpp_sys_rs::Il2CppClass, ()>;
/// Nullable Class handle
pub type Il2CppClassRef = Ref<il2cpp_sys_rs::Il2CppClass, ()>;

impl Il2CppClass {
    /// Returns the class name
    #[inline]
    pub const fn name(self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.as_ref().name) }
    }

    /// Returns the class type
    ///
    /// # Panics
    ///
    /// Panics if the type pointer is null
    #[track_caller]
    #[inline]
    pub fn type_(self) -> NonNullRef<Il2CppType, ()> {
        Ref::new(unsafe { il2cpp_class_get_type(self.as_ptr()) as _ }).unwrap_non_null()
    }

    /// Returns the declaring type, if the class is nested
    #[inline]
    pub const fn declaring_type(self) -> Option<Self> {
        Self::from_ptr(self.as_ref().declaringType)
    }

    /// Returns the class namespace
    #[inline]
    pub const fn namespace(self) -> &'static CStr {
        if self.as_ref().namespaze.is_null() {
            c""
        } else {
            unsafe { CStr::from_ptr(self.as_ref().namespaze) }
        }
    }

    /// Returns a human-readable fully qualified name.
    ///
    /// Combines namespace and class name with a dot.
    #[inline]
    pub fn full_name(self) -> Cow<'static, str> {
        // todo: include the declaring type ?

        let ns = self.namespace().to_string_lossy();
        let name = self.name().to_string_lossy();
        if ns.is_empty() {
            name
        } else {
            format!("{ns}.{name}").into()
        }
    }

    /// Returns the base class of this type, if it inherits one
    ///
    /// # Panics
    ///
    /// Panics if the parent class pointer is null
    #[track_caller]
    #[inline]
    pub const fn parent(self) -> Option<Self> {
        Self::from_ptr(self.as_ref().parent)
    }

    /// Returns the class image
    ///
    /// # Panics
    ///
    /// Panics if the image pointer is null
    #[track_caller]
    #[inline]
    pub const fn image(self) -> Il2CppImage {
        Il2CppImage::from_ptr(self.as_ref().image as _).unwrap()
    }

    /// Returns `true` if the class is initialized
    #[inline]
    pub fn is_initialized(self) -> bool {
        unsafe { il2cpp_class_is_inited(self.as_ptr()) }
    }

    /// Returns `true` if the class is generic
    #[inline]
    pub fn is_generic(self) -> bool {
        unsafe { il2cpp_class_is_generic(self.as_ptr()) }
    }

    /// Returns `true` if the class is inflated
    #[inline]
    pub fn is_inflated(self) -> bool {
        unsafe { il2cpp_class_is_inflated(self.as_ptr()) }
    }

    /// Returns `true` if the class is a value type
    #[inline]
    pub fn is_value_type(self) -> bool {
        unsafe { il2cpp_class_is_valuetype(self.as_ptr()) }
    }

    /// Returns `true` if the class is blittable
    #[inline]
    pub fn is_blittable(self) -> bool {
        unsafe { il2cpp_class_is_blittable(self.as_ptr()) }
    }

    /// Returns `true` if the class is abstract
    #[inline]
    pub fn is_abstract(self) -> bool {
        unsafe { il2cpp_class_is_abstract(self.as_ptr()) }
    }

    /// Returns `true` if the class is an interface
    #[inline]
    pub fn is_interface(self) -> bool {
        unsafe { il2cpp_class_is_interface(self.as_ptr()) }
    }

    /// Returns `true` if the class is an enum type
    #[inline]
    pub fn is_enum(self) -> bool {
        unsafe { il2cpp_class_is_enum(self.as_ptr()) }
    }

    /// Checks whether this class is a subclass of `other`
    ///
    /// - Traverses the parent class chain to see if `other` is a base class
    /// - If `check_interfaces` is `true`:
    ///   - Also checks interfaces implemented by `self` and its parent classes
    ///
    /// # Arguments
    ///
    /// * `other` – Class to test as a potential base
    /// * `check_interfaces` – Whether to include interfaces in the check
    #[inline]
    pub fn is_subclass_of(self, other: Self, check_interfaces: bool) -> bool {
        unsafe { il2cpp_class_is_subclass_of(self.as_ptr(), other.as_ptr(), check_interfaces) }
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
    pub fn array_element_size(self) -> usize {
        unsafe { il2cpp_class_array_element_size(self.as_ptr()) as usize }
    }

    /// Returns the number of instance fields
    #[inline]
    pub fn instance_field_count(self) -> usize {
        unsafe { il2cpp_class_num_fields(self.as_ptr()) }
    }

    /// Returns all field definitions of the class.
    ///
    /// Each [`FieldInfo`] describes a declared field, including static and instance ones.
    #[inline]
    pub const fn fields(self) -> &'static [FieldInfo] {
        unsafe {
            slice::from_raw_parts(
                self.as_ref().fields as _,
                self.as_ref().field_count as usize,
            )
        }
    }

    /// Returns all property definitions of the class.
    ///
    /// Each [`PropertyInfo`] describes a declared property, including static and instance ones.
    #[inline]
    pub const fn properties(self) -> &'static [PropertyInfo] {
        unsafe {
            slice::from_raw_parts(
                self.as_ref().properties as _,
                self.as_ref().property_count as usize,
            )
        }
    }

    /// Returns all method definitions of the class.
    ///
    /// Each [`MethodInfo`] represents a declared method, including inherited and generic ones.
    #[inline]
    pub const fn methods(self) -> &'static [MethodInfo] {
        unsafe {
            slice::from_raw_parts(
                self.as_ref().methods as _,
                self.as_ref().method_count as usize,
            )
        }
    }

    /// Finds a field by name
    ///
    /// # Arguments
    ///
    /// * `name` - Field name
    ///
    /// # Returns
    ///
    /// FieldInfo handle if found, otherwise `None`
    #[inline]
    pub fn find_field(self, name: &CStr) -> Option<FieldInfo> {
        FieldInfo::from_name(self, name)
    }

    /// Finds a property by name
    ///
    /// # Arguments
    ///
    /// * `name` - Property name
    ///
    /// # Returns
    ///
    /// PropertyInfo handle if found, otherwise `None`
    #[inline]
    pub fn find_property(self, name: &CStr) -> Option<PropertyInfo> {
        PropertyInfo::from_name(self, name)
    }

    /// Finds a method by name and arity
    ///
    /// # Arguments
    ///
    /// * `name` - Simple method name. For generic **definitions**, **do not** include the arity suffix.
    /// * `arity` - Number of parameters
    ///
    /// # Returns
    ///
    /// Method handle if found, otherwise `None`
    #[inline]
    pub fn find_method(self, name: &CStr, arity: i32) -> Option<MethodInfo> {
        MethodInfo::from_name(self, name, arity)
    }
}

impl Il2CppClass {
    /// Finds a class by namespace and name
    ///
    /// # Arguments
    ///
    /// * `image` - Image containing the class
    /// * `namespace` - Namespace of the class, empty for global
    /// * `name` - Simple class name.
    ///   For generic **definitions**, include the arity suffix (e.g. `List`1`, `Dictionary`2`).
    ///   Do **not** include type arguments here. For nested types, use `Outer`1/Inner`2`.
    ///
    /// # Returns
    ///
    /// Class handle if found, otherwise `None`
    #[inline]
    pub(crate) fn from_name(image: Il2CppImage, namespace: &CStr, name: &CStr) -> Option<Self> {
        unsafe {
            Ref::new(il2cpp_class_from_name(
                image.as_ptr(),
                namespace.as_ptr(),
                name.as_ptr(),
            ))
            .non_null()
        }
    }
}

impl fmt::Display for Il2CppClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl fmt::Debug for Il2CppClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Il2CppClass")
            .field("name", &self.name().to_string_lossy())
            .field("namespace", &self.namespace().to_string_lossy())
            .field("declaring_type", &self.declaring_type())
            .field("image", &self.image())
            .finish()
    }
}
