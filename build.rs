extern crate cc;
extern crate counted_array;
#[macro_use]
extern crate lazy_static;

use counted_array::counted_array;
use std::collections::HashMap;
use std::env;

struct BuildRule {
    include_directries: Vec<String>,
    cpp_files: Vec<String>,
}

lazy_static! {
    static ref LIBRARY_TARGETS: HashMap<String, BuildRule> = {
        let mut m = HashMap::new();
        m.insert("field".to_string(), BuildRule{
            include_directries: vec![ "./libField".to_string() ],
            cpp_files: vec![ format!("{}/loadField.cpp", "./libField")],
        });
        m
    };
}

counted_array!(const CFLAGS: [&str; _] = [
    "-v",
    "-g",
    "-Wall",
    "-Wextra",
    "-std=c++17",
]);

counted_array!(const LINK_LIBRARY_NAMES: [&str; _] = [
    "field",
]);

const LIBRARY_DIR: &str = "lib";

fn main() {
    let project_dir = env::current_dir().unwrap();

    for library in LINK_LIBRARY_NAMES {
        println!("cargo:rustc-link-lib=static={}", library);
    }

    println!(
        "cargo:rustc-link-search=native={}/{}",
        project_dir.display(),
        LIBRARY_DIR
    );
    println!("cargo:rustc-link-lib=c++");
    println!(
        "cargo:rerun-if-changed={}/libField/*",
        project_dir.display()
    );

    for (lib, rule) in LIBRARY_TARGETS.iter() {
        let mut build = cc::Build::new();
        build.cpp(true).warnings(true);

        for cflag in CFLAGS {
            build.flag(cflag);
        }

        for file in rule.cpp_files.iter() {
            build.file(file);
        }

        for path in rule.include_directries.iter() {
            build.include(path);
        }

        let libname = format!("lib{}.a", lib);
        build.compile(&libname);
    }
}
