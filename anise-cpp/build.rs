// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cpp")
        .compile("anise-cpp");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cpp");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
