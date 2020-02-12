use std::f32;

struct Complex {
  real: i32,
  imaginary: i32,
}

impl Complex {
  fn new (R: i32, I: i32) -> Complex
  {
    return Complex { real: R , imaginary: I };
  }

  fn to_string (&self)
  {
    println!("{}+{}i",self.real,self.imaginary);
  }

  fn add(val1: Complex, val2: Complex ) -> Complex
  {
    return Complex { 
      real: val1.real+val2.real, 
      imaginary: val1.imaginary+val2.imaginary
    };
  } 

  fn times_ten (&mut self)
  {
    self.real      = self.real*10;
    self.imaginary = self.imaginary*10;
  }

  fn abs_val (&self) -> f32
  {
    let result = self.real*self.real + self.imaginary*self.imaginary;
    return f32::sqrt(result as f32);
  }
}

pub fn run()
{
  let mut c1 = Complex::new(4,3);
  let c2     = Complex::new(5,6);
  c1.to_string();
  c2.to_string();
  println!("c1's abs val is {}", c1.abs_val());
  c1.times_ten();
  c1.to_string();
  let c3 = Complex::add(c1,c2);
  c3.to_string();
}