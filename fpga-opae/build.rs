use flate2::read::GzDecoder;
use std::{env, path::PathBuf};
use tar::Archive;

const OPAE_VERSION: &str = "2.0.4-1";
const OPAE_REPOSITORY: &str = "https://github.com/OPAE/opae-libs";

fn main() {
    println!("cargo:rustc-link-lib=opae-c");

    let opae_download_url = format!("{}/archive/{}.tar.gz", OPAE_REPOSITORY, OPAE_VERSION);
    let opae_dir = format!("opae-libs-{}", OPAE_VERSION);

    // Setup output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    if !out_dir.join(&opae_dir).exists() {
        // Download opae
        let download = reqwest::blocking::get(&opae_download_url).expect("opae download failed");
        // Extract to output directory
        Archive::new(GzDecoder::new(download))
            .unpack(&out_dir)
            .expect("opae tarball extract failed");
    }

    let include_dir = out_dir.join(&opae_dir).join("include");
    let include_dir = include_dir.display();

    bindgen::Builder::default()
        .header(format!("{}/opae/fpga.h", include_dir))
        .clang_arg(format!("-I{}", include_dir))
        .allowlist_function("fpga.*")
        .allowlist_type("fpga.*")
        .allowlist_var("fpga.*")
        .rustified_enum(".*")
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings");
}
