use rlass::class_file::ClassFileVersion;

#[test]
fn version_supports_test() {
    let first = ClassFileVersion::latest();
    let second = ClassFileVersion::new(52, 0);

    assert!(first.supports(second));
    assert!(!second.supports(first));
    assert!(first.supports(first));
}
