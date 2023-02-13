#![cfg_attr(
    not(feature = "contracts"),
    feature(proc_macro_hygiene, stmt_expr_attributes)
)]

use creusot_contracts::*;

#[requires(m != 0u32)]
#[ensures(result.0 * m + result.1 == n)]
#[ensures(result.1 < m)]
pub fn div_rem(n: u32, m: u32) -> (u32, u32) {
    let mut div = 0;
    let mut rem = n;
    #[invariant(inv , div * m + rem == n)]
    while rem >= m {
        rem -= m;
        div += 1;
    }
    (div, rem)
}
