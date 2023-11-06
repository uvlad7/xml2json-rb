use std::{env, process};

fn main() {
    // println!("cargo:rerun-if-env-changed=RBCONFIG_sitearch");
    // println!("cargo:rerun-if-env-changed=RBCONFIG_host_cpu");
    // println!("cargo:rerun-if-env-changed=RBCONFIG_host_os");
    let sitearch = env::var("DEP_RB_RBCONFIG_SITEARCH");
    match sitearch {
        Ok(value) => {
            if value.contains("java") {
                println!("cargo:rustc-cfg=jruby");
            } else {
                println!("cargo:rustc-cfg=mri");
            }
        }
        Err(_) => {
            eprintln!("DEP_RB_RBCONFIG_SITEARCH is not defined");
            process::exit(1);
        }
    }
}