extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let blis_types = vec![
        "arch_t", "cntl_t", "cntx_t", "conj_t", "dcomplex", "diag_t", "dim_t", "doff_t", "dom_t",
        "gint_t", "guint_t", "inc_t", "ind_t", "num_t", "obj_t", "prec_t", "rntm_t", "scomplex",
        "side_t", "siz_t", "struc_t", "trans_t", "uplo_t",
    ];
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(&format!("-I{}", std::env::var("DEP_BLIS_INCLUDE").unwrap()))
        .generate_inline_functions(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("bli_.*")
        .allowlist_var("BLIS_.*");
    for t in blis_types {
        builder = builder.allowlist_type(t);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
