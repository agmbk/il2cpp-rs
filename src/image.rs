//! Il2CppImage

use crate::{Il2CppAssembly, Il2CppClass, NonNullRef, Ref};
use il2cpp_sys_rs::{il2cpp_get_corlib, il2cpp_image_get_class, il2cpp_image_get_class_count};
use std::ffi::CStr;
use std::fmt;

/// Image handle
pub type Il2CppImage = NonNullRef<il2cpp_sys_rs::Il2CppImage, ()>;
/// Nullable Image handle
pub type Il2CppImageRef = Ref<il2cpp_sys_rs::Il2CppImage, ()>;

impl Il2CppImage {
    /// Returns the image name
    #[inline]
    pub const fn name(self) -> &'static CStr {
        // Safety: `name` is never null
        unsafe { CStr::from_ptr(self.as_ref().name) }
    }

    /// Returns the image name without file extension
    #[inline]
    pub const fn name_no_ext(self) -> &'static CStr {
        // Safety: `nameNoExt` is never null
        unsafe { CStr::from_ptr(self.as_ref().nameNoExt) }
    }

    /// Returns the parent assembly of the image
    ///
    /// # Panics
    ///
    /// Panics if the parent assembly pointer is null
    #[track_caller]
    #[inline]
    pub const fn assembly(self) -> Il2CppAssembly {
        Il2CppAssembly::from_ptr(self.as_ref().assembly).unwrap()
    }

    /// Returns the image token
    #[inline]
    pub const fn token(self) -> u32 {
        self.as_ref().token
    }

    /// Iterator over all classes in the image
    #[inline]
    pub fn classes(self) -> AssemblyClassIter {
        let len = unsafe { il2cpp_image_get_class_count(self.as_ptr()) };
        AssemblyClassIter {
            image: self,
            index: 0,
            len,
        }
    }

    /// Finds a class by namespace and name
    ///
    /// # Arguments
    ///
    /// * `namespace` - Namespace of the class, empty for global
    /// * `name` - Simple class name.
    ///   For generic **definitions**, include the arity suffix (e.g. `List`1`, `Dictionary`2`).
    ///   Do **not** include type arguments here. For nested types, use `Outer`1/Inner`2`.
    ///
    /// # Returns
    ///
    /// Class handle if found, otherwise `None`
    #[inline]
    pub fn find_class(self, namespace: &CStr, name: &CStr) -> Option<Il2CppClass> {
        Il2CppClass::from_name(self, namespace, name)
    }
}

impl Il2CppImage {
    /// Returns the core library image
    ///
    /// # Panics
    ///
    /// Panics if the `corlib` image pointer is null
    #[track_caller]
    #[inline]
    pub fn corlib() -> Self {
        unsafe { Self::from_ptr(il2cpp_get_corlib() as _).unwrap() }
    }
}

/// Iterator over all classes in an image
pub struct AssemblyClassIter {
    /// Parent image reference
    image: Il2CppImage,
    /// Current index
    index: usize,
    /// Total number of classes in the image
    len: usize,
}

impl Iterator for AssemblyClassIter {
    type Item = NonNullRef<il2cpp_sys_rs::Il2CppClass, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let class = unsafe { il2cpp_image_get_class(self.image.as_ptr(), self.index) };
        self.index += 1;
        Il2CppClass::from_ptr(class as _)
    }
}

impl fmt::Display for Il2CppImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl fmt::Debug for Il2CppImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Il2CppImage")
            .field("name", &self.name().to_string_lossy())
            .field("assembly", &self.assembly().name())
            .field("typeCount", &self.as_ref().typeCount)
            .field("exportedTypeCount", &self.as_ref().exportedTypeCount)
            .field("customAttributeCount", &self.as_ref().customAttributeCount)
            .field("token", &self.as_ref().token)
            .field("dynamic", &self.as_ref().dynamic)
            .finish()
    }
}
