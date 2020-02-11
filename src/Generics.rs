use std::f32;

pub fn run()
{

}

struct Person 
{
  name: &'static str,
  id:   i32
}

struct Pair<T>
{
  first:  T,
  Second: T
}

fn pair()
{
  let magic_pair: Pair<u32> = Pair {first: 7, second: 42};
}