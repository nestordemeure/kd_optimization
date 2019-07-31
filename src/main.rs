mod hypercube;
mod statistics;
mod kd_tree;
mod algorithm;

fn main()
{
   {
      let nb_iter = 30;
      let hypercube = vec![(-32.768, 32.768), (-32.768, 32.768)];
      let f = argmin_testfunctions::ackley;
      let (coordinate, value) = algorithm::minimize(f, hypercube, nb_iter);
      println!("Finished! found {} in {:?}", value, coordinate);
   }
}
