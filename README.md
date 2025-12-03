# il2cpp-rs

**Safe Rust bindings for IL2CPP internals.**

`il2cpp-rs` is a zero-cost abstraction over the [IL2CPP](https://docs.unity3d.com/Manual/IL2CPP.html) runtime.  
It bridges Unity's managed C# with Rust through a minimal API for interacting with IL2CPP runtime.

---

## Features

- **Safe abstractions** for core IL2CPP handles (`Class`, `MethodInfo`, `FieldInfo`, `PropertyInfo`, `Image`, etc.)
- **Strongly typed wrappers** over `il2cpp_sys_rs` (raw FFI)
- **Rust APIs** for method invocation, property access, and reflection
- **Minimal unsafe boilerplate** in user code
- **Reduced code complexity** over raw bindings

---

## Getting Started

Add to your `Cargo.toml`:

```toml
[dependencies]
il2cpp-rs = "0.1"
````

Configure [il2cpp-sys-rs](https://github.com/agmbk/il2cpp-sys-rs?tab=readme-ov-file#usage) with the target Unity
version.

Then import and initialize:

```rust
use il2cpp_rs::{Il2CppImage, Il2CppClass};

fn main() {
    // Access the core library (mscorlib)
    let corlib = Il2CppImage::corlib();
    println!("Loaded image: {}", corlib.name().to_string_lossy());

    // Find a class by namespace and name
    if let Some(class) = corlib.find_class(c"System", c"String") {
        println!("Found class: {}", class.full_name());
    }
}
```

---

## Example: Invoking methods

```rust
use il2cpp_rs::{Il2CppImage, NonNullRef, Ref};

fn main() {
    let image = Il2CppImage::corlib();
    let class = image.find_class(c"System", c"Math").unwrap();

    // Find the first method named `Abs` with one parameter.
    // Usually, this resolves to the `System.Int16` overload.
    let method = class.find_method(c"Abs", 1).unwrap();

    // Print the method signature "System.Int16 Abs(System.Int16 value)"
    println!("{}", method.signature());

    // Invoke the static method with its arguments
    let mut args: [*mut std::ffi::c_void; _] = [&mut -10_i16 as *mut _ as _];
    // The result is a boxed `Il2CppObject`
    let result = method.invoke::<()>(Ref::null(), &mut args).unwrap();

    // Unbox the managed result into a native value
    let value = unsafe {
        il2cpp_rs::sys::il2cpp_object_unbox(result.as_ptr()) as *mut i16
    };
    assert_eq!(value.as_ref(), Some(&10));
}
```

---

## Architecture

* **`il2cpp_sys_rs`**
  Low-level unsafe FFI bindings (generated from C headers)

* **`il2cpp-rs`**
  High-level safe abstractions built on top of `il2cpp_sys_rs`

---

## License

* [MIT License](LICENSE)
