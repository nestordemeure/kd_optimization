//! This is a blackbox optimization algorithm that partition the space using a [kd-tree](https://en.wikipedia.org/wiki/K-d_tree) 
//! and performs the search using [Monte-Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search).
//! 
//! The ideal target function would have a single minimum, no useless input dimenssions and be very **noisy**.

mod hypercube;
mod statistics;
mod kd_tree;
mod algorithm;
pub use algorithm::{maximize, minimize};
pub use hypercube::Coordinate;