use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{utils, link, coinbuilder};

const LIB_NAME: &str = "Osi";

fn main() {
    println!("cargo:rerun-if-changed={}_lib_sources.txt", LIB_NAME.to_ascii_lowercase());
    println!("cargo:rerun-if-changed=CARGO_{}_STATIC", LIB_NAME.to_ascii_uppercase());
    println!("cargo:rerun-if-changed=CARGO_{}_SYSTEM", LIB_NAME.to_ascii_uppercase());

    link::link_lib_system_if_defined(LIB_NAME);

    if !Path::new(&format!("{}/AUTHORS", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let mut config = coinbuilder::init_builder();

    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let mut includes_dir = vec![
        format!("{}/Osi", src_dir),
    ];

    let mut lib_sources = include_str!("osi_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/Osi/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["OSI".to_string()];

    if cfg!(feature = "cplex") {
        lib_sources.push(format!("{}/OsiCpx/OsiCpxSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiCpx", src_dir));
        coinflags.push("OSICPX".to_string());
    }
    if cfg!(feature = "glpk") {
        lib_sources.push(format!("{}/OsiGlpk/OsiGlpkSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiGlpk", src_dir));
        coinflags.push("OSIGLPK".to_string());
    }
    if cfg!(feature = "gurobi") {
        lib_sources.push(format!("{}/OsiGrb/OsiGrbSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiGrb", src_dir));
        coinflags.push("OSIGRB".to_string());
    }
    if cfg!(feature = "mosek") {
        lib_sources.push(format!("{}/OsiMsk/OsiMskSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiMsk", src_dir));
        coinflags.push("OSIMSK".to_string());
    }
    if cfg!(feature = "soplex") {
        lib_sources.push(format!("{}/OsiSpx/OsiSpxSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiSpx", src_dir));
        coinflags.push("OSISPX".to_string());
    }
    if cfg!(feature = "xpress") {
        lib_sources.push(format!("{}/OsiXpr/OsiXprSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiXpr", src_dir));
        coinflags.push("OSIXPR".to_string());
    }

    coinbuilder::print_metedata(includes_dir.clone(), coinflags.clone());

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("CoinUtils");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile("Osi");
}
