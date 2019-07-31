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
      right: Box<KDTree>, stat_right:Statistics, left: Box<KDTree>, stat_left:Statistics
   }
}

impl KDTree 
{
   /// explores a KDTree
   fn explore(self, f: impl Fn(Coordinate) -> f64, rng: &mut impl Rng) -> (KDTree, Point)
   {
      match self 
      {
         KDTree::Empty(hypercube) =>
         {
            let point = hypercube.sample(f);
            let tree = KDTree::Singleton{hypercube, point:point.clone()};
            (tree, point)
         }
         KDTree::Singleton{hypercube, point} =>
         {
            let (right, left) = hypercube.split();
            if right.contains(&point.coordinate)
            {
               let right = KDTree::Singleton{hypercube:right, point:point};
               let (left, point) = KDTree::Empty(left).explore(f, rng);
               let tree = KDTree::Split{right: Box::new(right), left: Box::new(left)};
               (tree, point)
            }
            else
            {
               let left = KDTree::Singleton{hypercube:left, point:point};
               let (right, point) = KDTree::Empty(right).explore(f, rng);
               let tree = KDTree::Split{right: Box::new(right), left: Box::new(left)};
               (tree, point)
            }
         }
         KDTree::Split{right, stat_right, left, stat_left} =>
         {
            let n_root = stat_right.n + stat_left.n;
            if stat_right.score(rng, n_root) > stat_left.score(rng, n_root)
            {
               let (right, point) = right.explore(f, rng);
               let tree = KDTree::Split{right:Box::new(right), left};
               (tree, point)
            }
            else
            {
               let (left, point) = left.explore(f, rng);
               let tree = KDTree::Split{right, left:Box::new(left)};
               (tree, point)
            }
         }
      }
   }
}
