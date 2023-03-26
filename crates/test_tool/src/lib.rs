use std::{
    env,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

pub use pretty_assertions::{assert_eq, assert_ne};

pub fn compart_to_snapshot(result: String, fn_name: &str) {
    let base_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let snapshot_dir = base_dir.join("tests").join("snapshots");
    if !snapshot_dir.as_path().exists() {
        create_dir_all(snapshot_dir.clone()).unwrap();
    }
    let snapshop_file_name = snapshot_dir.join(format!("{}_output", fn_name));
    if env::var("UPDATE").is_ok() {
        let _ = File::create(snapshop_file_name)
            .unwrap()
            .write_all(result.as_bytes())
            .unwrap();
    } else {
        let mut out_put = String::new();
        if snapshop_file_name.as_path().exists() {
            let _ = File::open(snapshop_file_name)
                .unwrap()
                .read_to_string(&mut out_put);
            assert_eq!(result, out_put);
        } else {
            let _ = File::create(snapshop_file_name)
                .unwrap()
                .write_all(result.as_bytes())
                .unwrap();
        }
    }
}
