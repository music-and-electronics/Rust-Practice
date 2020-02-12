pub fn run()
{
  let x =3;
  let y =0;

  if y == 0 {panic!("Division by 0 is occurred");}
  println!("{}", div(x, y));
}

fn div(x: i32, y: i32) -> f32
{
  return (x / y) as f32;
}