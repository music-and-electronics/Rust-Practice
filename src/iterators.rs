pub fn run()
{
  even_pow3();
}

fn rng()
{
  let mut rng = 0..7;
  println!("> {:?}", rng.next());
  println!("> {:?}", rng.next());
  for n in rng
  {
    print!("{} - ",n);
  }
}

fn aliens()
{
  let aliens = ["Cherfer","Fynock","Shirack","Zuxu"];
  for alien in aliens.iter()
  {
    print!("{} / ",alien);
  }
}

fn super_iter()
{
  let rng = 0..1000_000;
  for i in rng
  {
    println!("{}",i);
  }
}

fn consumer()
{
  let mut rng = 0..1000;
  let forty_two = rng.find(|n| *n >= 42);
  println!("{:?}", forty_two);
}

fn even_filter()
{
  let mut rng = 0..1000;
  let rng_even = rng.filter(|n| is_even(*n))
                    .collect::<Vec<i32>>();
  println!("{:?}", rng_even);
}

fn is_even(n: i32) -> bool
{
  n % 2 == 0
}

fn even_pow3()
{
  let mut rng = 0..1000;
  let rng_even_pow3 = rng.filter(|n| is_even(*n))
                         .map(|n| n * n * n)
                         .collect::<Vec<i32>>();
  println!("{:?}",rng_even_pow3);
}