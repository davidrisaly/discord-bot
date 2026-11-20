#![cfg(test)]

use super::*;

#[test]
fn test_level() {
    assert_eq!(lv::calc_level(2500), 5);
}

#[test]
fn test_exp() {
    assert_eq!(lv::calc_exp(5), 2500);
}
