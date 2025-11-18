//! Il2CppImage

use crate::{Il2CppAssembly, NonNullRef, Ref};
use il2cpp_sys_rs::il2cpp_get_corlib;
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
        unsafe { Ref::new(il2cpp_get_corlib() as _).unwrap_non_null() }
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
            .field("nameNoExt", &self.name_no_ext().to_string_lossy())
            .field("assembly", &self.assembly().name())
            .field("typeCount", &self.as_ref().typeCount)
            .field("exportedTypeCount", &self.as_ref().exportedTypeCount)
            .field("customAttributeCount", &self.as_ref().customAttributeCount)
            .field("token", &self.as_ref().token)
            .field("dynamic", &self.as_ref().dynamic)
            .finish()
    }
}
