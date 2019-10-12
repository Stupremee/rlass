use crate::class_file::ClassFileVersion;
use rlass::class_file::ClassFileVersion;

#[test]
fn version_supports_test() {
    let first = ClassFileVersion::new(53, 0);
    let second = ClassFileVersion::new(52, 1);
    let third = ClassFileVersion::new(52, 0);

    assert!(first.supports(second));
    assert!(first.supports(third));
    assert!(second.supports(third));
    assert_ne!(second.supports(first));
    assert!(first.supports(first));
    assert_ne!(third.supports(second));
    assert_ne!(third.supports(first));
}
