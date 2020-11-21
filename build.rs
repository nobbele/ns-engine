use std::{env, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=resources/*");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    
    fs_extra::dir::copy(
        "resources",
        out_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        },
    )
    .unwrap();
}