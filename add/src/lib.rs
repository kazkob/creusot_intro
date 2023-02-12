#![cfg_attr(
    not(feature = "contracts"),
    feature(proc_macro_hygiene, stmt_expr_attributes)
)]

use creusot_contracts::*;

#[requires(i <= 1000u32 && j <= 1000u32)]
#[ensures(result == i + j)]
pub fn add(i : u32, j : u32) -> u32 {
    i + j
}

#[requires(i <= 1000u32)]
#[ensures(result == i + 1u32)]
pub fn inc(i : u32) -> u32 {
    add(1, i)
}
