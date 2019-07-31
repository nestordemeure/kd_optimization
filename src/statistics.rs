//! Algorithm used to choose a branch or another.
use rand::Rng;
use statrs::distribution::{Normal, Continuous, Univariate};

pub struct Statistics
{
   sum: f64,
   sum_squares: f64,
   min: f64,
   max: f64,
   pub n: usize
}

impl Statistics
{
   /// produces a new set of statistics
   pub fn new(value: f64) -> Statistics
   {
      let sum = value;
      let sum_squares = value * value;
      let min = value;
      let max = value;
      let n = 1;
      Statistics { sum, sum_squares, min, max, n }
   }

   /// adds a value to the statistics
   pub fn update(&mut self, value: f64)
   {
      self.sum += value;
      self.sum_squares += value * value;
      self.n += 1;
      if value > self.max
      {
         self.max = value;
      }
      else if value < self.min
      {
         self.min = value
      }
   }

   /// computes the mean of the scores so far
   fn mean(&self) -> f64
   {
      self.sum / (self.n as f64)
   }

   /// computes the variance of the socre so far
   fn variance(&self) -> f64
   {
      if self.n < 2
      {
         0.
      }
      else
      {
         let mean = self.mean();
         let n_unbiaised = self.n as f64 - 1.;
         let var = (self.sum_squares / n_unbiaised) - (mean * mean);
         var.abs()
      }
   }

   /// computes a score using the thompson max formula
   fn score_thompson(&self, rng: &mut impl Rng) -> f64
   {
      let n = self.n as f64;
      let mean = self.mean();
      let sup = (n + 1.).ln() * self.max;
      mean + (sup - mean) * rng.gen::<f64>()
   }

   /// computes a score using the ucb-tuned formula
   fn score_ucb(&self, n_root: usize) -> f64
   {
      let n_root = n_root as f64;
      let n = self.n as f64;
      let c = self.variance() + (2. * n_root.ln() / n).sqrt();
      self.mean() + (c * n_root.ln() / n).sqrt()
   }

   /// computes a score using the [expected improvement formula](https://thuijskens.github.io/2016/12/29/bayesian-optimisation/)
   fn score_expected_improvement(&self) -> f64
   {
      let std = self.variance().sqrt();
      if std == 0.
      {
         0.
      }
      else
      {
         let normal = Normal::new(0.0, 1.0).unwrap();
         let mean = self.mean();
         let z = (mean - self.max) / std;
         (mean - self.max) * normal.cdf(z) + std * normal.pdf(z)
      }
   }

   /// computes the score of the statistics, the higher the better
   pub fn score(&self, rng: &mut impl Rng, n_root: usize) -> f64
   {
      self.score_thompson(rng)
      //self.score_ucb(n_root)
      //self.score_expected_improvement()
   }
}
