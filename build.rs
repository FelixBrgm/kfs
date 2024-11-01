use core::convert::From;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let multiboot_object_file = format!("{}/multiboot_header.o", out_dir);

    assemble_multiboot_header(&multiboot_object_file);
    link(&multiboot_object_file);
}

fn assemble_multiboot_header(multiboot_object_file: &str) {
    let status = Command::new("nasm")
        .args(&[
            "-f",
            "elf64",
            "multiboot_header.s",
            "-o",
            &multiboot_object_file,
        ])
        .status()
        .expect("failed to assemble multiboot_header.s");
    if !status.success() {
        panic!("nasm could not assemble multiboot_header.s")
    }
    println!("cargo:rerun-if-changed=multiboot_header.s");
}

fn link(multiboot_object_file: &str) {
    let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join("x86_64-unknown-none")
        .join("release");
    let deps_dir = target_dir.join("deps");

    let obj_file = std::fs::read_dir(deps_dir)
        .expect("failed to read deps directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "o" {
                Some(path)
            } else {
                None
            }
        })
        .next()
        .expect("no object file found in deps directory");

    let status = Command::new("ld")
        .args(&[
            "--nmagic",
            "--output=kernel",
            "--script=linker.ld",
            &multiboot_object_file,
            obj_file.to_str().unwrap(),
            "-z",
            "noexecstack",
        ])
        .status()
        .expect("failed to link kernel object file with assembled multiboot header");
    if !status.success() {
        panic!("ld failed during the linking process");
    }
}
