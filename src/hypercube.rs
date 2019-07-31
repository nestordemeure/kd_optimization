use rand::Rng;
use ordered_float::OrderedFloat;

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
   pub coordinates: Vec<(f64, f64)>
}

impl Hypercube
{
   /// samples a point from an hypercube (using the given fucntion to evaluate it)
   pub fn sample(&self, rng: &mut impl Rng) -> Coordinate
   {
      self.coordinates.iter().map(|(inf, sup)| rng.gen_range(inf, sup)).collect()
   }

   /// splits an hypercube into two sub-cubes along its largest dimenssion
   pub fn split(self) -> (Hypercube, Hypercube)
   {
      // finds the index of the largest coordinate
      // splits the hypercube into two along this coordinate
      let dim_split = self.coordinates
                          .iter()
                          .enumerate()
                          .max_by_key(|(_, (inf, sup))| OrderedFloat(sup - inf))
                          .map(|(i, _)| i)
                          .unwrap();
      let mut coordinates_left = self.coordinates;
      let mut coordinates_right = coordinates_left.clone();
      let (inf, sup) = coordinates_left[dim_split];
      let mid = (inf + sup) / 2.;
      coordinates_left[dim_split] = (inf, mid);
      coordinates_right[dim_split] = (mid, sup);
      (Hypercube { coordinates: coordinates_left }, Hypercube { coordinates: coordinates_right })
   }

   /// returns true if the hypercube contains the given coordinates
   pub fn contains(&self, coordinate: &Coordinate) -> bool
   {
      self.coordinates.iter().zip(coordinate.iter()).all(|((inf, sup), x)| (x >= inf) && (x <= sup))
   }
}
