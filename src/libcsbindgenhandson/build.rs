fn main(){
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("csbindgenhandson")
        .csharp_namespace("CsbindgenHandsOn.Native")
        .generate_csharp_file("../CsbindgenHandsOn/Native/NativeMethods.g.cs")
        .unwrap();
}