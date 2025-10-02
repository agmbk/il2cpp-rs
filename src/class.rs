use crate::{NonNullRef, Ref};

/// Non-null managed Class handle
pub type Il2CppClass = NonNullRef<il2cpp_sys_rs::Il2CppClass, ()>;
/// Nullable managed Class handle
pub type Il2CppClassRef = Ref<il2cpp_sys_rs::Il2CppClass, ()>;

impl Il2CppClass {}
