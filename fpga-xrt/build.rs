use flate2::read::GzDecoder;
use std::{env, path::PathBuf};
use tar::Archive;

const XRT_VERSION: &str = "b7f8b62eecdba9caf906893cd22aa5dd39795766";
const XRT_REPOSITORY: &str = "https://github.com/Xilinx/XRT";

fn main() {
    println!("cargo:rustc-link-lib=xrt_core");
    println!("cargo:rustc-link-lib=xrt_coreutil");

    let xrt_download_url = format!("{}/archive/{}.tar.gz", XRT_REPOSITORY, XRT_VERSION);
    let xrt_dir = format!("XRT-{}", XRT_VERSION);

    // Setup output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    if !out_dir.join(&xrt_dir).exists() {
        // Download xrt
        let download = reqwest::blocking::get(&xrt_download_url).expect("xrt download failed");
        // Extract to output directory
        Archive::new(GzDecoder::new(download))
            .unpack(&out_dir)
            .expect("xrt tarball extract failed");
    }

    let include_dir = out_dir.join(&xrt_dir).join("src/runtime_src/core/include");
    let include_dir = include_dir.display();

    bindgen::Builder::default()
        .header(format!("{}/xrt.h", include_dir))
        .header(format!("{}/xrt/xrt_bo.h", include_dir))
        .header(format!("{}/xrt/xrt_device.h", include_dir))
        .header(format!("{}/xrt/xrt_kernel.h", include_dir))
        .header(format!("{}/xrt/xrt_uuid.h", include_dir))
        .header(format!("{}/experimental/xrt_ini.h", include_dir))
        .header(format!("{}/experimental/xrt_error.h", include_dir))
        .header(format!("{}/xrt_error_code.h", include_dir))
        .clang_arg(format!("-I{}", include_dir))
        .allowlist_function("xrt.*")
        .allowlist_type("xrt.*")
        .allowlist_var("xrt.*")
        .allowlist_function("xcl.*")
        .allowlist_type("xcl.*")
        .allowlist_var("xcl.*")
        .rustified_enum(".*")
        .derive_debug(true)
        .derive_default(true)
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings");
}
