mod hypercube;
mod statistics;
mod kd_tree;
mod algorithm;

fn main()
{
   let nb_iter = 1000;

   {
      // this function is particularly noisy
      const DIM: usize = 5;
      let hypercube = vec![(-32.768, 32.768); DIM];
      let f = argmin_testfunctions::ackley;
      let (_coordinate, value) = algorithm::minimize(f, hypercube, nb_iter);
      println!("Finished! found {} (target:0)", value);
   }

   {
      let hypercube = vec![(0., 1.), (0., 1.)];
      let f = argmin_testfunctions::picheny;
      let (coordinate, value) = algorithm::minimize(f, hypercube, nb_iter);
      println!("Finished! found {} in {:?} (target:-3.3851993182036826)", value, coordinate);
   }

   {
      // this function has several similar minimums
      let hypercube = vec![(-5., 5.), (-5., 5.)];
      let f = argmin_testfunctions::himmelblau;
      let (coordinate, value) = algorithm::minimize(f, hypercube, nb_iter);
      println!("Finished! found {} in {:?} (target:0)", value, coordinate);
   }
}
