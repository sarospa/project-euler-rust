use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=config.json");
    println!("cargo:warning=Hello from build.rs");
    println!("cargo:warning=CWD is {:?}", env::current_dir().unwrap());
    println!("cargo:warning=OUT_DIR is {:?}", env::var("OUT_DIR").unwrap());
    println!("cargo:warning=CARGO_MANIFEST_DIR is {:?}", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=PROFILE is {:?}", env::var("PROFILE").unwrap());

    let output_path = env::var_os("OUT_DIR").unwrap();
    println!("cargo:warning=Calculated build path: {}", output_path.to_str().unwrap());

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src\\key.txt");
	println!("cargo:warning=Calculated input path: {}", input_path.to_str().unwrap());
	
    let output_path = Path::new(&output_path).join("key.txt");
    let res = std::fs::copy(input_path, output_path);
    println!("cargo:warning={:#?}",res)
}