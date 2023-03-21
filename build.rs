use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, link, utils};

const LIB_NAME: &str = "Osi";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    let want_system = utils::want_system(LIB_NAME);

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let mut coinflags = vec!["OSI".to_string()];

        let link_type = if utils::want_static(LIB_NAME) {
            "static".to_string()
        } else {
            "dylib".to_string()
        };

        if cfg!(feature = "osicpx") {
            println!("cargo:rustc-link-lib={}=OsiCpx", link_type);
            coinflags.push("OSICPX".to_string());
        }
        if cfg!(feature = "osiglpk") {
            println!("cargo:rustc-link-lib={}=OsiGlpk", link_type);
            coinflags.push("OSIGLPK".to_string());
        }
        if cfg!(feature = "osigrb") {
            println!("cargo:rustc-link-lib={}=OsiGrb", link_type);
            coinflags.push("OSIGRB".to_string());
        }
        if cfg!(feature = "osimsk") {
            println!("cargo:rustc-link-lib={}=OsiMsk", link_type);
            coinflags.push("OSIMSK".to_string());
        }
        if cfg!(feature = "osispx") {
            println!("cargo:rustc-link-lib={}=OsiSpx", link_type);
            coinflags.push("OSISPX".to_string());
        }
        if cfg!(feature = "osixpr") {
            println!("cargo:rustc-link-lib={}=OsiXpr", link_type);
            coinflags.push("OSIXPR".to_string());
        }

        coinbuilder::print_metedata(Vec::new(), coinflags);
        return;
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let mut includes_dir = vec![format!("{}/Osi", src_dir)];

    let mut lib_sources = include_str!("osi_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/Osi/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["OSI".to_string()];

    if cfg!(feature = "osicpx") {
        lib_sources.push(format!("{}/OsiCpx/OsiCpxSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiCpx", src_dir));
        coinflags.push("OSICPX".to_string());
    }
    if cfg!(feature = "osiglpk") {
        lib_sources.push(format!("{}/OsiGlpk/OsiGlpkSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiGlpk", src_dir));
        coinflags.push("OSIGLPK".to_string());
    }
    if cfg!(feature = "osigrb") {
        lib_sources.push(format!("{}/OsiGrb/OsiGrbSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiGrb", src_dir));
        coinflags.push("OSIGRB".to_string());
    }
    if cfg!(feature = "osimsk") {
        lib_sources.push(format!("{}/OsiMsk/OsiMskSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiMsk", src_dir));
        coinflags.push("OSIMSK".to_string());
    }
    if cfg!(feature = "osispx") {
        lib_sources.push(format!("{}/OsiSpx/OsiSpxSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiSpx", src_dir));
        coinflags.push("OSISPX".to_string());
    }
    if cfg!(feature = "osixpr") {
        lib_sources.push(format!("{}/OsiXpr/OsiXprSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiXpr", src_dir));
        coinflags.push("OSIXPR".to_string());
    }

    coinbuilder::print_metedata(includes_dir.clone(), coinflags.clone());

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("CoinUtils");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile("Osi");
}
