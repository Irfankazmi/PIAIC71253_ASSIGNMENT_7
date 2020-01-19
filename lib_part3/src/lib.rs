pub mod shapes
{
   pub mod cylinder
    {   
       pub fn volume(radius:f64,height:f64)
        {
            let pi:f64 = 22.0/7.0;
            let vol = pi*(radius*radius)*  height;
            println!("Volume of cylinder is = {}",vol);
        }
    }
}
