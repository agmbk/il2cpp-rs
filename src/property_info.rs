//! PropertyInfo

use crate::{ExceptionRef, Il2CppClass, MethodInfo, NonNullRef, Ref};
use il2cpp_sys_rs::{
    il2cpp_class_get_property_from_name, il2cpp_type_get_name, Il2CppObject, Il2CppType,
};
use std::ffi::{c_void, CStr};
use std::fmt;

/// PropertyInfo handle
pub type PropertyInfo = NonNullRef<il2cpp_sys_rs::PropertyInfo, ()>;
/// Nullable PropertyInfo handle
pub type PropertyInfoRef = Ref<il2cpp_sys_rs::PropertyInfo, ()>;

impl PropertyInfo {
    /// Returns the property name
    #[inline]
    pub const fn name(self) -> &'static CStr {
        // Safety: `name` is never null
        unsafe { CStr::from_ptr(self.as_ref().name) }
    }

    /// Returns the property type
    ///
    /// # Panics
    ///
    /// Panics if neither getter nor setter is defined
    #[inline]
    pub const fn type_(self) -> NonNullRef<Il2CppType, ()> {
        if let Some(getter) = self.getter() {
            getter.return_type()
        } else {
            let parameters = self
                .setter()
                .expect("Property must have either a getter or a setter")
                .parameters();

            parameters
                .last()
                .expect("Setters always have one parameter")
                .unwrap_non_null()
        }
    }

    /// Returns the parent class of the property
    ///
    /// # Panics
    ///
    /// Panics if the parent class pointer is null
    #[track_caller]
    #[inline]
    pub const fn parent(self) -> Il2CppClass {
        Il2CppClass::from_ptr(self.as_ref().parent as _).unwrap()
    }

    /// Returns the getter method, if any
    #[track_caller]
    #[inline]
    pub const fn getter(self) -> Option<MethodInfo> {
        Ref::new(self.as_ref().get as _).non_null()
    }

    /// Reads the property value from an object instance
    ///
    /// # Safety
    ///
    /// The caller must ensure the target `object` is the exact parent object
    /// of this property instance
    ///
    /// # Arguments
    ///
    /// * `object` - Instance that declares this property
    pub unsafe fn get<T>(
        self,
        object: NonNullRef<T, ()>,
    ) -> Result<Ref<Il2CppObject, ()>, ExceptionRef> {
        if let Some(getter) = self.getter() {
            getter.invoke(object.into(), &mut [])
        } else {
            Err(Ref::null())
        }
    }

    /// Returns the setter method, if any
    #[track_caller]
    #[inline]
    pub const fn setter(self) -> Option<MethodInfo> {
        Ref::new(self.as_ref().set as _).non_null()
    }

    /// Write the property value in an object instance
    ///
    /// # Safety
    ///
    /// The caller must ensure the target `object` is the exact parent object
    /// of this property instance
    ///
    /// # Arguments
    ///
    /// * `object` - Instance that declares this property
    /// * `value` - Pointer to the new value
    pub unsafe fn set<T>(
        self,
        object: NonNullRef<T, ()>,
        value: *mut c_void,
    ) -> Result<Ref<Il2CppObject, ()>, ExceptionRef> {
        if let Some(setter) = self.setter() {
            setter.invoke(object.into(), &mut [value])
        } else {
            Err(Ref::null())
        }
    }

    /// Returns the raw property flags
    #[inline]
    pub const fn flags(self) -> u32 {
        self.as_ref().attrs
    }

    /// Returns the property token
    #[inline]
    pub const fn token(self) -> u32 {
        self.as_ref().token
    }
}

impl PropertyInfo {
    /// Finds a property by name
    ///
    /// # Arguments
    ///
    /// * `class` - Class containing the property
    /// * `name` - Property name
    ///
    /// # Returns
    ///
    /// PropertyInfo handle if found, otherwise `None`
    #[inline]
    pub(crate) fn from_name(class: Il2CppClass, name: &CStr) -> Option<Self> {
        unsafe {
            Ref::new(il2cpp_class_get_property_from_name(class.as_ptr(), name.as_ptr()) as _)
                .non_null()
        }
    }
}

impl fmt::Display for PropertyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl fmt::Debug for PropertyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PropertyInfo")
            .field("name", &self.name().to_string_lossy())
            .field("type", &unsafe {
                CStr::from_ptr(il2cpp_type_get_name(self.type_().as_ptr())).to_string_lossy()
            })
            .finish()
    }
}
