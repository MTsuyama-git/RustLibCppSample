use std::env;

fn main()
{
    let project_dir = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}/libField", project_dir.display());
    println!("cargo:rustc-link-lib=static=field");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rerun-if-changed={}/libField/*", project_dir.display());
}