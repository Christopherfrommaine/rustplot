#[allow(unused_imports)] // inputs are used, but doesn't detect it?
use crate::helper::math::{*, non_nan_type::*};

#[test]
fn non_nan_wrapper_test_1() {
    let nn = NonNanWrapper::from(0.);
    assert_eq!(nn.value(), 0.);
}

#[test]
fn non_nan_wrapper_test_2() {
    let nn = NonNanWrapper::from(12);
    assert_eq!(nn.value(), 12);
}

#[test]
fn non_nan_wrapper_test_3() {
    let nn1 = NonNanWrapper::from(0.);
    let nn2 = NonNanWrapper::from(12.);
    assert!(nn1 < nn2);
}

#[test]
fn non_nan_wrapper_test_4() {
    assert!(std::panic::catch_unwind(|| NonNanWrapper::from(f64::NAN)).is_err());
}

#[test]
fn ciel_div_test_1() {
    assert_eq!(ciel_div(1, 2), 1);
}

#[test]
fn ciel_div_test_2() {
    assert_eq!(ciel_div(8, 4), 2);
}

#[test]
fn ciel_div_test_3() {
    assert!(std::panic::catch_unwind(|| ciel_div(1, 0)).is_err());
}

#[test]
fn bin_to_u8_test_1() {
    assert_eq!(bin_to_u8(vec![true, true, true, true, true, true, true, true]), 0b11111111);
}

#[test]
fn bin_to_u8_test_2() {
    assert_eq!(bin_to_u8(vec![true, true, true, true]), 0b00001111);
}

#[test]
fn bin_to_u8_test_3() {
    assert_eq!(bin_to_u8(vec![]), 0b00000000);
}

#[test]
fn bin_to_u8_test_4() {
    assert_eq!(bin_to_u8(vec![true, true, true, true, true]), 0b11111);
}

#[test]
fn bin_to_u8_test_5() {
    assert_eq!(bin_to_u8(vec![true, true, true]), 0b111);
}