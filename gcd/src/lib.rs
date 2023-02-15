#![cfg_attr(
    not(feature = "contracts"),
    feature(proc_macro_hygiene, stmt_expr_attributes)
)]

use creusot_contracts::*;

#[predicate]
fn divide(n: Int, d: Int) -> bool {
    pearlite! { exists <q : Int> n == q * d }
}

#[predicate]
fn is_common_divisor(a: Int, b: Int, d: Int) -> bool {
    d != 0 && divide(a, d) && divide(b, d)
}

#[logic]
#[ensures(
    forall <p : Int, q : Int, d : Int>
    d != 0 ==> divide(p, d) ==> divide(q, d) ==> divide(p + q, d))]
fn lemma_divide_add() {}

#[logic]
#[ensures(
    forall <p : Int, q : Int, d : Int>
    d != 0 ==> divide(p, d) ==> divide(q, d) ==> divide(p - q, d))]
fn lemma_divide_sub() {}

#[logic]
#[ensures(
    forall <p : Int, q : Int, d : Int>
    d != 0 ==> divide(q, d) ==> divide(p * q, d))]
fn lemma_divide_mult() {}

#[requires(@a != 0 && @b != 0)]
#[ensures(is_common_divisor(@a, @b, @result))]
#[ensures(forall<d:u64> is_common_divisor(@a, @b, @d) ==> divide(@result, @d))]
pub fn gcd(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = if a < b { (a, b) } else { (b, a) };

    #[invariant(cd, forall<d:Int> is_common_divisor(@x, @y, d) == is_common_divisor(@a, @b, d))]
    #[invariant(le_xy, @x <= @y)]
    #[invariant(y_pos, 0 < @y)]
    while x != 0 {
        proof_assert!(lemma_divide_add(); true);
        proof_assert!(lemma_divide_sub(); true);
        proof_assert!(lemma_divide_mult(); true);

        proof_assert!(forall <d:Int> is_common_divisor(@x, @y % @x, d) ==> divide(@y, d));
        proof_assert!(@y % @x  == @y - (@y / @x) * @x);
        proof_assert!(
            forall <d:Int> is_common_divisor(@x, @y, d) ==> divide(@y % @x, d));
        y = y % x;
        std::mem::swap(&mut x, &mut y);
    }

    proof_assert!(is_common_divisor(@x, @y, @y));
    y
}
