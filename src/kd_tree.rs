use crate::hypercube::*;
use crate::statistics::*;
use rand::Rng;

/// represents a partitionning of the space
enum KDTree
{
   /// this hypercube has never been explored
   Empty(Hypercube),
   /// this hypercube has only been explored once
   Singleton
   {
      hypercube: Hypercube, point: Point
   },
   /// this hypercube has been splited in two along a dimension, you need to choose a branch
   Split
   {
      right: Box<KDTree>, stat_right: Statistics, left: Box<KDTree>, stat_left: Statistics
   }
}

impl KDTree
{
   /// creates a new, empty, kdTree to explre the given hypercube
   fn new(coordinates: Vec<(f64, f64)>) -> KDTree
   {
      let hypercube = Hypercube { coordinates };
      KDTree::Empty(hypercube)
   }

   /// explores a KDTree using the given function and random number generator
   /// returns the point generated and an updated kdtree
   fn explore(self, f: impl Fn(&Coordinate) -> f64, rng: &mut impl Rng) -> (KDTree, Point)
   {
      match self
      {
         KDTree::Empty(hypercube) =>
         {
            let coordinate = hypercube.sample(rng);
            let value = f(&coordinate);
            let point = Point { coordinate, value };
            let tree = KDTree::Singleton { hypercube, point: point.clone() };
            (tree, point)
         }
         KDTree::Singleton { hypercube, point } =>
         {
            let (right, left) = hypercube.split();
            if right.contains(&point.coordinate)
            {
               let stat_right = Statistics::new(point.value);
               let right = KDTree::Singleton { hypercube: right, point: point };
               let (left, point) = KDTree::Empty(left).explore(f, rng);
               let stat_left = Statistics::new(point.value);
               let tree =
                  KDTree::Split { right: Box::new(right), stat_right, left: Box::new(left), stat_left };
               (tree, point)
            }
            else
            {
               let stat_left = Statistics::new(point.value);
               let left = KDTree::Singleton { hypercube: left, point: point };
               let (right, point) = KDTree::Empty(right).explore(f, rng);
               let stat_right = Statistics::new(point.value);
               let tree =
                  KDTree::Split { right: Box::new(right), stat_right, left: Box::new(left), stat_left };
               (tree, point)
            }
         }
         KDTree::Split { right, mut stat_right, left, mut stat_left } =>
         {
            let n_root = stat_right.n + stat_left.n;
            if stat_right.score(rng, n_root) > stat_left.score(rng, n_root)
            {
               let (right, point) = right.explore(f, rng);
               stat_right.update(point.value);
               let tree = KDTree::Split { right: Box::new(right), stat_right, left, stat_left };
               (tree, point)
            }
            else
            {
               let (left, point) = left.explore(f, rng);
               stat_left.update(point.value);
               let tree = KDTree::Split { right, stat_right, left: Box::new(left), stat_left };
               (tree, point)
            }
         }
      }
   }
}
