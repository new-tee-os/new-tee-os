use std::env;


fn main () {

    let sdk_dir = env::var("SGX_SDK")
                    .unwrap_or_else(|e| e.to_string());
    let is_sim = env::var("SGX_MODE")
                    .unwrap_or_else(|e| e.to_string());
    println!("cargo:rustc-link-search=native=../lib");
    println!("cargo:rustc-link-search=native={}/lib64", "/home/lx/Downloads/SGX/sgxsdk/");
    println!("cargo:rustc-link-search=native=../lib/mesalock-rt/");

    println!("cargo:rustc-link-lib=static=Enclave_u");
    println!("cargo:rustc-link-lib=static=test");

    match is_sim.as_ref() {
        "HW" => println!("cargo:rustc-link-lib=dylib=sgx_urts"),
        _    => println!("cargo:rustc-link-lib=dylib=sgx_urts_sim"),
    }
}
