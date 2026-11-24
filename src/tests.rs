#![cfg(test)]

use super::*;

#[test]
fn test_level() {
    assert_eq!(commands::leveling::_calc_level(2500), 5);
}

#[test]
fn test_exp() {
    assert_eq!(commands::leveling::_calc_exp(5), 2500);
}
