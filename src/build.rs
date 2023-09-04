use std::{
    env,
    fmt::Write as FmtWrite,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

macro_rules! include_shim {
    (
        $root:expr => {
            $(
                $name:expr $(,)?
            )*
        }
    ) => {
        [
            $(
                ($name, include_str!(concat!($root, "/", $name))),
            )*
        ]
    }
}

pub fn build() -> io::Result<impl Iterator<Item = String>> {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
    let shim_name = format!("po6_rt_{crate_name}_");
    let mut source = String::new();

    for (name, shim_source) in include_shim!(
        "shim" => {
            "assert.rs",
            "errno.rs",
            "malloc.rs",
            "stdlib.rs",
            "string.rs",
        }
    ) {
        let name = name.replace(".rs", "");
        let shim_source = shim_source.replace("po6_rt_", &shim_name);

        writeln!(source, "mod {name} {{ \n {shim_source} \n }}").unwrap();
    }

    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let out_path = out_dir.join("po6_rt.rs");
    let headers_path = out_dir.join("po6_rt_include");

    File::create(out_path)?.write_all(source.as_bytes())?;
    fs::create_dir_all(&headers_path)?;

    for (name, source) in include_shim!(
        "include" => {
            "assert.h",
            "errno.h",
            "inttypes.h",
            "malloc.h",
            "pthread.h",
            // "stdint.h",
            "stdio.h",
            "stdlib.h",
            "string.h",
            "unistd.h",
        }
    ) {
        let mut source = source.to_string();

        if name == "stdint.h" {
            let word_size = std::mem::size_of::<usize>() * 8;

            writeln!(source, "#define __WORDSIZE {word_size}").unwrap()
        }

        File::create(headers_path.join(name))?
            .write_all(source.replace("po6_rt_", &shim_name).as_bytes())?;
    }

    Ok([headers_path.to_str().unwrap().to_string()].into_iter())
}
