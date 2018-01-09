extern crate renderer;

use std::path::PathBuf;

pub fn get_test_path(relative_path: &[&str]) -> String {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests");
    for component in relative_path {
        test_path.push(component);
    }

    test_path.to_str().unwrap().to_string()
}

pub fn import_nano_moscow() -> String {
    let bin_file = get_test_path(&["osm", "nano_moscow.bin"]);
    renderer::geodata::importer::import(&get_test_path(&["osm", "nano_moscow.osm"]), &bin_file)
        .unwrap();

    bin_file
}
