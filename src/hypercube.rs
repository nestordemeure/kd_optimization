/// defines a coordinate in space
pub type Coordinate = Vec<f64>;

/// defines an alreaddy evaluated point
#[derive(Clone)]
pub struct Point
{
   pub coordinate: Coordinate,
   pub value: f64
}

/// defines the hypercube in which to conduct the search
pub struct Hypercube
{
   coordinates: Vec<(f64, f64)>
}

impl Hypercube
{
   /// samples a point from an hypercube (using the given fucntion to evaluate it)
   pub fn sample(&self, f: impl Fn(Coordinate) -> f64) -> Point
   {
      unimplemented!()
   }

   /// splits an hypercube into two sub-cubes along its largest dimenssion
   pub fn split(self) -> (Hypercube, Hypercube)
   {
      unimplemented!()
   }

   /// returns true if the hypercube contains the given coordinates
   pub fn contains(&self, coordinate: &Coordinate) -> bool
   {
      self.coordinates.iter().zip(coordinate.iter()).all(|((inf, sup), x)| x >= inf && x <= sup)
   }
}
