use std::env;

fn main() {
    let syphon_out_dir =
        env::var("DEP_SYPHON_OUT_DIR").expect("Syphon output directory isn't set correctly.");
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}", &syphon_out_dir);
}
