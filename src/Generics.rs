use std::f32;

pub fn run()
{
  cal();
}

struct Person {
  name: &'static str,
  id:   i32
}

struct Pair<T> {
  first:  T,
  second: T
}

fn pair()
{
  let magic_pair: Pair<u32> = Pair {first: 7, second: 42};
  let pair_of_magicians: Pair<&str>  
      = Pair {first: "Gandalf", second: "Sauron" };
  let a = second(magic_pair);
  println!("Val is {:?}",a);
}

fn second<T> (pair: Pair<T>) -> T 
{
  return pair.second;
}

fn struct_mit_vec()
{
  let p1  = Person{ name: "James Aond", id: 7};
  let p2  = Person{ name: "Kames Bond", id: 14};
  let p3  = Person{ name: "Lames Cond", id: 21};
  let op1: Option<Person> = Some(p1);
  let pvec: Vec<Person> = vec![p2,p3];

  for person in pvec.iter()
  {
    println!("{}",person.name);
  }
}


fn cal()
{
  let m = sqroot(-1.0);

  match m 
  {
    Ok(sq)   => println!("The square root of 42 is {}",sq),
    Err(str) => println!("{}",str),
  }
}

fn sqroot(r: f32) -> Result<f32,String>
{
  if r < 0.0
  {
    return Err("Numner cannot be negative".to_string());
  }
  return Ok(f32::sqrt(r));
}