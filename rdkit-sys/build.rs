use std::env;
use std::path::PathBuf;

fn main() {
    for name in [
        "DOCS_RS",
        "RDKIT_PREFIX",
        "RDKIT_INCLUDE_DIR",
        "RDKIT_LIB_DIR",
        "CONDA_PREFIX",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if !matches!(target_os.as_str(), "linux" | "macos") {
        panic!("rdkit-sys supports Linux and macOS; {target_os}/{target_arch} is not tested");
    }
    let prefix = env::var_os("RDKIT_PREFIX")
        .or_else(|| {
            env::var_os("CARGO_FEATURE_DYNAMIC_LINKING_FROM_CONDA").map(|_| {
                env::var_os("CONDA_PREFIX")
                    .expect("activate a conda environment or set RDKIT_PREFIX")
            })
        })
        .map(PathBuf::from)
        .unwrap_or_else(|| match (target_os.as_str(), target_arch.as_str()) {
            ("macos", "aarch64") => PathBuf::from("/opt/homebrew"),
            ("macos", _) => PathBuf::from("/usr/local"),
            _ => PathBuf::from("/usr"),
        });
    let include_dir = env::var_os("RDKIT_INCLUDE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| prefix.join("include/rdkit"));
    let lib_dir = env::var_os("RDKIT_LIB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| prefix.join("lib"));
    if !include_dir.join("GraphMol/ROMol.h").is_file() {
        panic!(
            "RDKit headers not found at {}. Install librdkit-dev or set RDKIT_PREFIX / RDKIT_INCLUDE_DIR; see docs/building.md",
            include_dir.display()
        );
    }

    let mut rust_files: Vec<_> = std::fs::read_dir("src/bridge")
        .expect("read src/bridge")
        .map(|entry| entry.expect("read bridge entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter(|path| !path.ends_with("mod.rs"))
        .collect();
    rust_files.sort();
    let mut cpp_files = Vec::new();
    println!("cargo:rerun-if-changed=src/bridge");
    println!("cargo:rerun-if-changed=wrapper");
    for file in &rust_files {
        let stem = file.file_stem().unwrap().to_str().unwrap();
        let source = PathBuf::from(format!("wrapper/src/{stem}.cc"));
        let header = PathBuf::from(format!("wrapper/include/{stem}.h"));
        assert!(source.is_file(), "{} must exist", source.display());
        assert!(header.is_file(), "{} must exist", header.display());
        for path in [file, &source, &header] {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        cpp_files.push(source);
    }

    cxx_build::bridges(rust_files)
        .files(cpp_files)
        .include(include_dir)
        .include(prefix.join("include"))
        .include(prefix.join("include/eigen3"))
        .include(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .std("c++20")
        .warnings(false)
        .compile("rdkit_cxx_bridge");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    for lib in [
        "DataStructs",
        "Depictor",
        "Descriptors",
        "FileParsers",
        "Fingerprints",
        "GraphMol",
        "MolStandardize",
        "RDGeneral",
        "RDGeometryLib",
        "ScaffoldNetwork",
        "SmilesParse",
        "SubstructMatch",
    ] {
        println!("cargo:rustc-link-lib=dylib=RDKit{lib}");
    }
    println!("cargo:rustc-link-lib=dylib=boost_serialization");
}
