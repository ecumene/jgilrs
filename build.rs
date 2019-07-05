use std::{
    env,
    path::{Path, PathBuf},
};

use rust_swig::{JavaConfig, LanguageConfig};

fn main() {
    env_logger::init();

    let out_dir = env::var("OUT_DIR").unwrap();

    gen_jni_bindings(&out_dir);

    let in_src = Path::new("src").join("main").join("rust").join("lib.rs.in");
    let out_src = Path::new(&out_dir).join("lib.rs");
    let swig_gen = rust_swig::Generator::new(LanguageConfig::JavaConfig(JavaConfig::new(
        Path::new("src").join("main").join("java").join("xyz").join("ecumene").join("jgilrs"),
        "xyz.ecumene.jgilrs".into(),
    )));
    swig_gen.expand("libgilrs", &in_src, &out_src);
}

fn gen_jni_bindings(out_dir: &str) {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME env variable not set");

    let java_include_dir = Path::new(&java_home).join("include");

    let target = env::var("TARGET").expect("target env var not set");
    let java_sys_include_dir = java_include_dir.join(if target.contains("windows") {
        "win32"
    } else if target.contains("darwin") {
        "darwin"
    } else {
        "linux"
    });

    let include_dirs = [java_include_dir, java_sys_include_dir];
    println!("jni include dirs {:?}", include_dirs);

    let jni_h_path =
        search_file_in_directory(&include_dirs[..], "jni.h").expect("Can't find jni.h");

    gen_binding(
        &include_dirs[..],
        &jni_h_path,
        &Path::new(out_dir).join("jni_c_header.rs"),
    )
    .expect("gen_binding failed");
}

fn search_file_in_directory<P: AsRef<Path>>(dirs: &[P], file: &str) -> Result<PathBuf, ()> {
    for dir in dirs {
        let dir = dir.as_ref().to_path_buf();
        let file_path = dir.join(file);
        if file_path.exists() && file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(())
}

fn gen_binding<P: AsRef<Path>>(
    include_dirs: &[P],
    c_file_path: &Path,
    output_rust: &Path,
) -> Result<(), String> {
    let mut bindings: bindgen::Builder = bindgen::builder().header(c_file_path.to_str().unwrap());
    bindings = include_dirs.iter().fold(bindings, |acc, x| {
        acc.clang_arg("-I".to_string() + x.as_ref().to_str().unwrap())
    });

    let generated_bindings = bindings
        .generate()
        .map_err(|_| "Failed to generate bindings".to_string())?;
    generated_bindings
        .write_to_file(output_rust)
        .map_err(|err| err.to_string())?;

    Ok(())
}
