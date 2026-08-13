//! Graphcal TIR: typed intermediate representation and dimension checking.

#[warn(clippy::arithmetic_side_effects)]
pub mod dim_check;
pub mod map_literal_fact;
pub mod materialized_shape;
pub mod presentation;
pub mod typed;
