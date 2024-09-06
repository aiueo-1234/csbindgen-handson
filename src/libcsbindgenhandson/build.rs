fn main(){
    bindgen::Builder::default()
        .header("src/c/myMath.h")
        .generate().unwrap()
        .write_to_file("src/myMath.rs").unwrap();

    cc::Build::new()
        .file("src/c/myMath.c")
        .try_compile("myMath").unwrap();
    
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("csbindgenhandson")
        .csharp_namespace("CsbindgenHandsOn.Native")
        .generate_csharp_file("../CsbindgenHandsOn/Native/NativeMethods.g.cs")
        .unwrap();
}