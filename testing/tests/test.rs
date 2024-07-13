use serial_test::serial;
use testing::{test_kernel, Mode};

const BASIC_EXPECTED_PREFIX: &'static str = "    INFO: Hello world! (basic";

#[test]
#[serial]
fn basic_bios() {
    let output = test_kernel!("basic", Mode::Bios);

    assert!(output[0].starts_with(BASIC_EXPECTED_PREFIX), "{:?}", output);
}

#[test]
#[serial]
fn basic_uefi() {
    let output = test_kernel!("basic", Mode::Uefi);

    assert!(output[0].starts_with(BASIC_EXPECTED_PREFIX), "{:?}", output);
}

const FORMAT_EXPECTED: &'static str = "**** Hello world! ****";

#[test]
#[serial]
fn custom_format_uefi() {
    let output = test_kernel!("format", Mode::Uefi);

    assert_eq!(output[0], FORMAT_EXPECTED, "{:?}", output);
}

#[test]
#[serial]
fn custom_format_bios() {
    let output = test_kernel!("format", Mode::Bios);

    assert_eq!(output[0], FORMAT_EXPECTED, "{:?}", output);
}

#[test]
#[serial]
fn multi_line_uefi() {
    let output = test_kernel!("multi", Mode::Uefi);

    for i in 0..100 {
        assert_eq!(
            output[i],
            format!("Hello world! {}", 99 - i),
            "{:?}",
            output
        );
    }
}

#[test]
#[serial]
fn multi_line_bios() {
    let output = test_kernel!("multi", Mode::Bios);

    for i in 0..100 {
        assert_eq!(
            output[i],
            format!("Hello world! {}", 99 - i),
            "{:?}",
            output
        );
    }
}
