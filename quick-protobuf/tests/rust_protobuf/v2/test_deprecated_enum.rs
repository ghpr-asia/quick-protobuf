// We just want to check that variant `Thing::C` exists. As long as this
// compiles without an error it should pass; we don't need any asserts
#[test]
fn test_deprecated_enum_with_add_deprecated_fields() {
    use super::test_deprecated_enum_with_add_deprecated_fields::*;

    let t = Thing::C;
}

#[test]
fn test_deprecated_enum_compile_fail_script() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/rust_protobuf/v2/test_deprecated_enum_compile_fail_script.rs");
}
