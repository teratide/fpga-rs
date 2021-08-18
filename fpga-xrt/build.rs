use cc::Build;
use glob::glob;
use std::{env, error::Error, ffi::OsStr, path::PathBuf, process::Command};

fn uuid_static() -> Result<(), Box<dyn Error>> {
    // Build and link static uuid.
    glob("util-linux/libuuid/src/*.c")?
        .map(Result::unwrap)
        .filter(|path| path.is_file())
        .filter(|path| {
            let file_name = path.file_name().unwrap();
            // Skip test file.
            file_name != OsStr::new("test_uuid.c") &&
            // This file produces some warnings when build without autoconf.
            // Skip because we don't need symbols from it.
            file_name != OsStr::new("gen_uuid.c")
        })
        .fold(Build::new(), |mut build, path| {
            build.file(path);
            build
        })
        // At least one method must be available so we pick nanosleep.
        .define("HAVE_NANOSLEEP", None)
        .file("util-linux/lib/randutils.c")
        .file("util-linux/lib/md5.c")
        .file("util-linux/lib/sha1.c")
        .include("util-linux/libuuid")
        .include("util-linux/include")
        .compile("uuid");

    Ok(())
}

fn boost_static() -> Result<(), Box<dyn Error>> {
    // Build and link static boost, using the script provided by XRT.
    // Static libs are installed in OUT_DIR/boost/xrt/lib.
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "xrt/src/runtime_src/tools/scripts/boost.sh -prefix {}/boost",
            env::var("OUT_DIR")?
        ))
        .spawn()?
        .wait_with_output()?;

    // Add path of static boost libs to linker search.
    println!(
        "cargo:rustc-link-search=native={}/boost/xrt/lib",
        env::var("OUT_DIR")?
    );

    // Set boot env for xrt build.
    env::set_var("XRT_BOOST_INSTALL", format!("{}/boost/xrt", env::var("OUT_DIR")?));

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    uuid_static()?;
    boost_static()?;

    let xrt_coreutil_static = cmake::Config::new("xrt/src")
        .env("XRT_BOOST_INSTALL", format!("{}/boost/xrt", env::var("OUT_DIR")?))
        .build_target("xrt_coreutil_static")
        .build();

    println!("cargo:rustc-link-search=native={}/build/runtime_src/core/common", xrt_coreutil_static.display());

    let xrt_core_static = cmake::Config::new("xrt/src")
        .env("XRT_BOOST_INSTALL", format!("{}/boost/xrt", env::var("OUT_DIR")?))
        .build_target("xrt_core_static")
        .build();

    println!("cargo:rustc-link-search=native={}/build/runtime_src/core/pcie/linux", xrt_core_static.display());

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    cxx_build::bridge("src/ffi.rs")
        .include(manifest_dir.join("xrt/src/runtime_src/core/include"))
        .include(env::var("OUT_DIR")?)
        .include(manifest_dir)
        .file("src/ffi.cc")
        .flag("-std=gnu++14")
        .compile("fpga-xrt");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/ffi.h");
    println!("cargo:rerun-if-changed=src/ffi.cc");

    // Link to xrt_core_static.
    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    println!("cargo:rustc-link-arg=-lxrt_core_static");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    
    println!("cargo:rustc-link-lib=static=xrt_coreutil_static");
    
    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    println!("cargo:rustc-link-arg=-lrt");
    println!("cargo:rustc-link-arg=-lpthread");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");

    // First link to static boost libs.
    println!("cargo:rustc-link-lib=static=boost_filesystem");
    println!("cargo:rustc-link-lib=static=boost_system");

    println!("cargo:rustc-link-arg=-lc");
    println!("cargo:rustc-link-arg=-lstdc++");

    // // Generate some bindings
    // bindgen::Builder::default()
    //     .header("xrt/src/runtime_src/core/include/xrt/xrt_uuid.h")
    //     .clang_arg("-Ixrt/src/runtime_src/core/include/")
    //     .allowlist_function("xrt.*")
    //     .allowlist_type("xrt.*")
    //     .allowlist_var("xrt.*")
    //     .allowlist_function("xcl.*")
    //     .allowlist_type("xcl.*")
    //     .allowlist_var("xcl.*")
    //     .rustified_enum(".*")
    //     .derive_debug(true)
    //     .derive_default(true)
    //     .impl_debug(true)
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("failed to generate bindings")
    //     .write_to_file(format!("{}/bindings.rs", env::var("OUT_DIR")?))
    //     .expect("failed to write bindings");

    Ok(())
}
