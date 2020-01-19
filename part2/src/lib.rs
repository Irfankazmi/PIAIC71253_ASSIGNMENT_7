 pub mod coordinate_system
{
   pub mod cartesian_system
    {
       pub fn slope(y2:f64,y1:f64,x2:f64,x1:f64)
       {
           println!("Result = {}",(y2-y1)/(x2-x1));

       }
    }
}
