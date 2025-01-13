//! This module contains shared proof logic for multiple `ProofExpr` / `ProofPlan` implementations.
mod bitwise_verification;
use bitwise_verification::{verify_constant_abs_decomposition, verify_constant_sign_decomposition};
#[cfg(test)]
mod bitwise_verification_test;
mod sign_expr;
pub(crate) use sign_expr::{prover_evaluate_sign, result_evaluate_sign, verifier_evaluate_sign};
#[allow(clippy::needless_range_loop)] // keep the loop for readability for now, refactor later
pub mod range_check;
#[cfg(all(test, feature = "blitzar"))]
/// this module is exposed from the test feature so we can access it in benches
pub mod range_check_test;
#[cfg(all(test, feature = "blitzar"))]
mod sign_expr_test;
