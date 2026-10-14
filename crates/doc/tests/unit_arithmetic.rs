//! Unit field arithmetic: `* /` chain left to right, and a sign after an operator is a sign.
// Integration tests: unwrapping and panicking on failure is fine here, unlike in shipped code (AGENTS.md › Robustness).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use vectorcraft_doc::{Unit, parse_number};

#[test]
fn multiply_then_divide() {
    assert_eq!(parse_number("100*5/2"), Some(250.0));
    assert_eq!(parse_number("10/2*3"), Some(15.0));
    assert_eq!(Unit::Points.parse("10*3/2"), Some(15.0));
    assert_eq!(Unit::Points.parse("2*3*4/6"), Some(4.0));
    assert_eq!(Unit::Points.parse("1in*2/4"), Some(36.0));
    assert_eq!(Unit::Pixels.parse("1080*1/2 px"), Some(540.0));
    assert_eq!(Unit::Points.parse("10+2*3/2"), Some(13.0));
    assert_eq!(Unit::Points.parse("10*3/0"), None);
}

#[test]
fn sign_after_an_operator() {
    assert_eq!(Unit::Points.parse("10*-2"), Some(-20.0));
    assert_eq!(Unit::Points.parse("10/-2"), Some(-5.0));
    assert_eq!(Unit::Points.parse("10--2"), Some(12.0));
    assert_eq!(Unit::Points.parse("-2*3"), Some(-6.0));
    assert_eq!(Unit::Points.parse("10-2*3"), Some(4.0));
    assert_eq!(Unit::Points.parse("10-2-3"), Some(5.0));
}
