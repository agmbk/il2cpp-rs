//! Il2CppAssembly

use il2cpp_sys_rs::{il2cpp_domain_get, il2cpp_domain_get_assemblies, Il2CppDomain};

use crate::{Il2CppImage, NonNullRef, Ref};
use std::ffi::CStr;
use std::{fmt, slice};

/// Assembly handle
pub type Il2CppAssembly = NonNullRef<il2cpp_sys_rs::Il2CppAssembly, ()>;
/// Nullable Assembly handle
pub type Il2CppAssemblyRef = Ref<il2cpp_sys_rs::Il2CppAssembly, ()>;

impl Il2CppAssembly {
    /// Returns the assembly name
    #[inline]
    pub const fn name(self) -> &'static CStr {
        // Safety: `name` is never null
        unsafe { CStr::from_ptr(self.as_ref().aname.name) }
    }

    /// Returns the image associated with this assembly
    ///
    /// # Panics
    ///
    /// Panics if the underlying image pointer is null
    #[track_caller]
    #[inline]
    pub const fn image(self) -> Il2CppImage {
        Il2CppImage::from_ptr(self.as_ref().image).unwrap()
    }
}

impl Il2CppAssembly {
    /// Finds an assembly by name
    ///
    /// # Arguments
    ///
    /// * `name` - Assembly name
    ///
    /// # Returns
    ///
    /// Assembly handle if found, otherwise `None`
    #[inline]
    pub fn from_name(name: &CStr) -> Option<Self> {
        unsafe {
            let domain: NonNullRef<Il2CppDomain, ()> =
                Ref::new(il2cpp_domain_get()).unwrap_non_null();

            let mut size = 0;
            let assemblies = il2cpp_domain_get_assemblies(domain.as_ptr(), &mut size);
            // Safety: null check is bypassed as assemblies should never be null
            let assemblies_slice = slice::from_raw_parts(assemblies as *mut Self, size);
            for &assembly in assemblies_slice {
                if assembly.name() == name {
                    return Some(assembly);
                }
            }

            None
        }
    }
}

impl fmt::Display for Il2CppAssembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl fmt::Debug for Il2CppAssembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Il2CppAssembly")
            .field("name", &self.name())
            .field("image", &self.image())
            .field(
                "referencedAssemblyStart",
                &self.as_ref().referencedAssemblyStart,
            )
            .field(
                "referencedAssemblyCount",
                &self.as_ref().referencedAssemblyCount,
            )
            .field(
                "version",
                &format_args!(
                    "{}.{}.{}.{}",
                    self.as_ref().aname.major,
                    self.as_ref().aname.minor,
                    self.as_ref().aname.build,
                    self.as_ref().aname.revision,
                ),
            )
            .finish()
    }
}
