//! Algorithm used to choose a branch or another.

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

   /// computes a score using the ucb-tuned formula
   /// the higher the better
   pub fn score(&self, n_root: usize) -> f64
   {
      let n_root = n_root as f64;
      let n = self.n as f64;
      let c = self.variance() + (2. * n_root.ln() / n).sqrt();
      self.mean() + (c * n_root.ln() / n).sqrt()
   }
}
