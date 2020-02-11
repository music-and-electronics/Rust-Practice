pub fn run()
{
  closure_func();
}

fn high_func()
{
  let mut strength = 26;
  println!("My tripled stringth equals {}",triples(strength));
  println!("My strength is still {}", strength);
  strength = triples(strength);
  println!("My strength is now {}", strength);
  strength = again(triples,strength);
  println!("I got so lucky to turn my strength into {}", strength);
}

fn triples(s : i32) -> i32 { s * 3 }

fn again <F: Fn(i32) -> i32> (f: F, s: i32) -> i32
{
  f(f(s))
}

fn closure_func()
{
  let mut strength = 78;
  let triples = |n| { 3 * n };
  strength= again(triples, strength);
  println!("My strength is now {}", strength);
}