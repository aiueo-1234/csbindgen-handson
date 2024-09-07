fn main() {
    bindgen::Builder::default()
        .header("src/c/myMath.h")
        .generate()
        .unwrap()
        .write_to_file("src/myMath.rs")
        .unwrap();

    cc::Build::new()
        .file("src/c/myMath.c")
        .try_compile("myMath")
        .unwrap();

    csbindgen::Builder::default()
        .input_bindgen_file("src/myMath.rs")
        .rust_method_prefix("cffi_")
        .rust_file_header("use super::myMath::*;")
        .csharp_entry_point_prefix("cffi_")
        .csharp_dll_name("csbindgenhandson")
        .csharp_namespace("CsbindgenHandsOn.Native")
        .csharp_class_name("CNativeMethodsMyMath")
        .generate_to_file(
            "src/myMath_ffi.rs",
            "../CsbindgenHandsOn/Native/CNativeMethodsMyMath.g.cs",
        )
        .unwrap();

    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("csbindgenhandson")
        .csharp_namespace("CsbindgenHandsOn.Native")
        .generate_csharp_file("../CsbindgenHandsOn/Native/NativeMethods.g.cs")
        .unwrap();

    bindgen::Builder::default()
        .header("src/c/myStack.h")
        .generate()
        .unwrap()
        .write_to_file("src/myStack.rs")
        .unwrap();

    cc::Build::new()
        .file("src/c/myStack.c")
        .try_compile("myStack")
        .unwrap();

    csbindgen::Builder::default()
        .input_bindgen_file("src/myStack.rs")
        .rust_method_prefix("cffi_")
        .rust_file_header("use super::myStack::*;")
        .csharp_entry_point_prefix("cffi_")
        .csharp_dll_name("csbindgenhandson")
        .csharp_namespace("CsbindgenHandsOn.Native")
        .csharp_class_name("CNativeMethodsMyStack")
        .generate_to_file(
            "src/myStack_ffi.rs",
            "../CsbindgenHandsOn/Native/CNativeMethodsMyStack.g.cs",
        )
        .unwrap();
}
