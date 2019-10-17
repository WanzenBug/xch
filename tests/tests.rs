use tempdir;
use libxch;
mod util;

#[test]
fn test_success() {
    let dir = tempdir::TempDir::new("test").expect("Could not create temporary directory");
    let file1 = dir.path().join("file1");
    let file2 = dir.path().join("file2");
    util::create_file_with_content(&file1, b"content1").expect("Could not create file in tempdir");
    util::create_file_with_content(&file2, b"content2").expect("Could not create file in tempdir");

    assert!(libxch::xch_non_atomic(&file1, &file2).is_ok());
    assert!(util::ensure_file_content(&file1, b"content2").expect("Could not read file"));
    assert!(util::ensure_file_content(&file2, b"content1").expect("Could not read file"));
}

#[test]
fn test_failure() {
    let dir = tempdir::TempDir::new("test").expect("Could not create temporary directory");
    let file1 = dir.path().join("file1");
    util::create_file_with_content(&file1, b"content1").expect("Could not create file in tempdir");

    assert!(libxch::xch_non_atomic(&file1, dir.path()).is_err());
    assert!(util::ensure_file_content(&file1, b"content1").expect("Could not read file"));
}
