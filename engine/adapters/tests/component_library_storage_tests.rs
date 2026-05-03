use hotsas_adapters::JsonComponentLibraryStorage;
use hotsas_ports::ComponentLibraryPort;
use std::path::PathBuf;

#[test]
fn load_builtin_library_returns_non_empty_library() {
    let storage = JsonComponentLibraryStorage;
    let lib = storage.load_builtin_library().unwrap();
    assert!(!lib.components.is_empty());
}

#[test]
fn save_library_to_path_writes_json() {
    let storage = JsonComponentLibraryStorage;
    let lib = storage.load_builtin_library().unwrap();
    let path = PathBuf::from("target/test_output/component_library.json");
    storage.save_library_to_path(&path, &lib).unwrap();
    assert!(path.exists());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("generic_resistor"));
}

#[test]
fn load_library_from_path_roundtrip_preserves_components() {
    let storage = JsonComponentLibraryStorage;
    let original = storage.load_builtin_library().unwrap();
    let path = PathBuf::from("target/test_output/component_library_roundtrip.json");
    storage.save_library_to_path(&path, &original).unwrap();
    let loaded = storage.load_library_from_path(&path).unwrap();
    assert_eq!(original.components.len(), loaded.components.len());
    assert_eq!(original.id, loaded.id);
}

#[test]
fn invalid_json_returns_error() {
    let storage = JsonComponentLibraryStorage;
    let path = PathBuf::from("target/test_output/invalid_library.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "not json").unwrap();
    let result = storage.load_library_from_path(&path);
    assert!(result.is_err());
}

#[test]
fn missing_file_returns_error() {
    let storage = JsonComponentLibraryStorage;
    let path = PathBuf::from("target/test_output/missing_library.json");
    let result = storage.load_library_from_path(&path);
    assert!(result.is_err());
}
