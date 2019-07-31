use crate::kd_tree::*;
use crate::hypercube::{Coordinate, Point};
use rand::FromEntropy;
use rand_xoshiro::Xoshiro256Plus;

/// finds a value that maximizes f in a given number of iterations
pub fn maximize(f: impl Fn(&[f64]) -> f64, hypercube: Vec<(f64, f64)>, nb_iter: usize) -> (Coordinate, f64)
{
   let mut rng = Xoshiro256Plus::from_entropy();
   let mut kdtree = KDTree::new(hypercube);
   let mut best_point = Point { coordinate: vec![], value: std::f64::NEG_INFINITY };

   for _iter in 0..nb_iter
   {
      let (new_tree, new_point) = kdtree.explore(&f, &mut rng);
      kdtree = new_tree;
      if new_point.value > best_point.value
      {
         best_point = new_point;
         //println!("{}: found {} in {:?}", _iter, best_point.value, best_point.coordinate);
      }
   }

   (best_point.coordinate, best_point.value)
}

/// finds a value that minimize f in a given number of iterations
pub fn minimize(f: impl Fn(&[f64]) -> f64, hypercube: Vec<(f64, f64)>, nb_iter: usize) -> (Coordinate, f64)
{
   let f: Box<dyn Fn(&[f64]) -> f64> = Box::new(|v| -f(v));
   let (coordinate, value) = maximize(f, hypercube, nb_iter);
   (coordinate, -value)
}
