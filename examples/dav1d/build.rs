use std::{env, path::PathBuf};

fn main() {
    let includes = ["dav1d", "dav1d/include", "include"]
        .into_iter()
        .map(str::to_string)
        .chain(po6::build().unwrap())
        .collect::<Vec<_>>();

    let mut build = cc::Build::new();

    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" {
        let supported = ["atomics", "bulk-memory", "mutable-globals"];

        for feature in env::var("CARGO_CFG_TARGET_FEATURE")
            .unwrap_or_default()
            .split(',')
        {
            if supported.contains(&feature) {
                build
                    .flag("-Xclang")
                    .flag("-target-feature")
                    .flag("-Xclang")
                    .flag(&format!("+{feature}"));
            }
        }
    }

    build
        .flag("-Wno-incompatible-pointer-types")
        .files(FILES.iter().map(|file| format!("dav1d/src/{file}")))
        .define("BITDEPTH", "8")
        .includes(&includes)
        .warnings(false)
        .compile("dav1d");

    bindgen::Builder::default()
        .header("dav1d/include/dav1d/dav1d.h")
        .layout_tests(false)
        .derive_default(true)
        .clang_args(includes.iter().flat_map(|path| ["-I", &path]))
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}

const FILES: &[&str] = &[
    "cdf.c",
    "cpu.c",
    "data.c",
    "decode.c",
    "dequant_tables.c",
    "getbits.c",
    "intra_edge.c",
    "itx_1d.c",
    "lf_mask.c",
    "lib.c",
    "log.c",
    "mem.c",
    "msac.c",
    "obu.c",
    "pal.c",
    "picture.c",
    "qm.c",
    "ref.c",
    "refmvs.c",
    "scan.c",
    "tables.c",
    "thread_task.c",
    "warpmv.c",
    "wedge.c",
    "cdef_apply_tmpl.c",
    "cdef_tmpl.c",
    "fg_apply_tmpl.c",
    "filmgrain_tmpl.c",
    "ipred_prepare_tmpl.c",
    "ipred_tmpl.c",
    "itx_tmpl.c",
    "lf_apply_tmpl.c",
    "loopfilter_tmpl.c",
    "looprestoration_tmpl.c",
    "lr_apply_tmpl.c",
    "mc_tmpl.c",
    "recon_tmpl.c",
];
