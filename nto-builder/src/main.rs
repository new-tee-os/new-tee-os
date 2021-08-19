use clap::{App,Arg};
use std::process::Command;
use std::env;
use std::collections::HashMap;
//cargo run  build sgx --mode=HW --sgxsdk=~/Downloads/SGX/sgxsdk
fn main() {
    let PATH: &'static str = env!("PATH");
    let filtered_env : HashMap<String, String> =
    env::vars().filter(|&(ref k, _)|
        true
    ).collect(); 
    let matches = App::new("nto")
        .version("1.0")
        .about("Compile and start tee os.")
        .subcommand(
            App::new("build")
            .about("Build kernel")
            .arg(
                Arg::new("ENV")
                .about("Tee environment")
                .required(true)
                .index(1)
            )
            .arg("--sgxsdk=[SGXSDK] 'SGXSDK path'")
            .arg("-S --stack-size=[SSIZE] 'Stack size of sgx enclave'")
            .arg("-H --heap-size=[HSIZE]  'Heap size of sgx enclave'")
            .arg("-m --mode=[SGXMODE] 'SGX mode, HW/SIM'")
        )        
        .subcommand(
            App::new("run")
            .about("Run kernel")
            .arg(
                Arg::new("ENV")
                .about("Tee environment")
                .required(true)
                .index(1)
            )
            .arg("--sgxsdk=[SGXSDK] 'SGXSDK path'")
            .arg("-u --user-bin=[UBPATH] 'Path of user bin'")
        )
        .get_matches();

        if let Some(ref matches)=matches.subcommand_matches("build"){
            match matches.value_of("ENV").unwrap(){
                "sgx"=>{
                    let sdk=matches.value_of("sgxsdk").unwrap_or("/opt/intel/sgxsdk");
                    let p=sdk.to_owned()+"/environment";
                    let mode=matches.value_of("mode").unwrap_or("SIM");
                    // for (key, value) in env::vars() {
                    //     println!("{}: {}", key, value);
                    // }          
                    Command::new("bash").arg("source").arg(p).envs(&filtered_env).output().expect("Source sdk environment.");
                    for (key, value) in &filtered_env{
                        println!("{}: {}", key, value);
                    }          
         
                    // println!("{:?}",);
                    Command::new("make")
                    .current_dir("/home/lx/Downloads/SGX/new-tee-os/sgx")//TODO######################
                    .envs(&filtered_env)
                    .args(&[" SGX_MODE=SIM","SGX_SDK=~/Downloads/SGX/sgxsdk"])
                    .spawn().expect("Make failure");
                    return;
                },
                "keystone"=>{

                    Command::new("cargo")
                    .args(&["build","--release"])
                    .envs(&filtered_env)
                    .current_dir("./keystone-rt")
                    .output()
                    .expect("build kernel");

                    Command::new("riscv64-unknown-elf-objcopy")
                    .args(&["-O","binary"])
                    .arg("target/riscv64gc-unknown-none-elf/release/keystone-rt")
                    .arg("keystone-rt.bin")
                    .current_dir("./keystone-rt")
                    .envs(&filtered_env)
                    .output()
                    .expect("kernel to bin");

                    Command::new("cargo")
                    .args(&["build","--release"])
                    .envs(&filtered_env)
                    .current_dir("./keystone-rt-runner")
                    .output()
                    .expect("build runner");
                    
                },
                "x86vm"=>{
                    Command::new("cargo")
                    .arg("install")
                    .args(&["--path=","x86-vmm-qemu"])
                    .envs(&filtered_env)
                    .spawn()
                    .expect("build runner");
                },
                _=>panic!("Unsupported environment!"),
            }
        }


        if let Some(ref matches)=matches.subcommand_matches("run"){
            match matches.value_of("ENV").unwrap(){
                "sgx"=>{
                    if let Some(sdk)=matches.value_of("SGXSDK"){
                        Command::new("bash").arg("source").arg(sdk.to_owned()+"/environment").envs(&filtered_env).output().expect("Source sdk environment.");                    
                    }
                    Command::new("app").current_dir("/home/lx/Downloads/SGX/new-tee-os/sgx/bin/").spawn().expect("run app");
                },
                "keystone"=>{
                    println!("Use qemu to test the kernel");
                },
                "x86vm"=>{
                    Command::new("cargo")
                    .arg("run")
                    .current_dir("x86-vm-kernel")
                    .envs(&filtered_env)
                    .spawn()
                    .expect("running x86");
                },
                _=>panic!("Unsupported environment!"),
            }
        }
        

}