#![cfg_attr(
    not(feature = "contracts"),
    feature(proc_macro_hygiene, stmt_expr_attributes)
)]

use creusot_contracts::*;

#[ensures(result == 1i32)]
#[ensures(^a == 0i32)]
#[ensures(^b == 1i32)]
pub fn hello(a : &mut i32, b : &mut i32) -> i32 {
    *a = 0;
    *b = 1;
    *a + *b
}
