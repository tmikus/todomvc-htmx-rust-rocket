use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=res/*");
    copy_to_output("src/css", &env::var("PROFILE").unwrap()).unwrap();
    copy_to_output("src/js", &env::var("PROFILE").unwrap()).unwrap();
    copy_to_output("src/views", &env::var("PROFILE").unwrap()).unwrap();
}
