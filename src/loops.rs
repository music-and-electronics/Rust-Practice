pub fn run()
{
  //controls
  let adult = true;
  let age   = if adult {"+18"} else {"-18"};
  println!("Age is {}", age);
  //conditions
  let dead   = false;
  let health = 48;
  if dead
  {   
      println!("Game over!");
      return;
  }
  else
  {
      println!("You are alive!");
  }
  if health >= 50
  {
      println!("fight");
  }
  else if health >=20
  {
      println!("Stop the battle!");
  }
  else
  {
      println!("Recover!");
  }
  //while loop
  let max_power = 10;
  let mut power = 1;
  while power < max_power
  {
      print!("{} ",power);
      power += 1;
  } 
  //just loop
  loop
  {
      power += 1;
      if power == 42
      {
          continue;
      }
      print!("{} ",power);
      if power == 50
      {
          print!("OK, that's enough for today");
          break;
      }
  }

  'outer : loop
  {
      println!("Entered the outer dunggeon - ");
      'inner : loop
      {
          println!("Entered the inner dungeon - ");
          break 'outer;
      }
      println!("This treasure can sadly never be reached! - ");
  }
  println!("Exited the outer dungeon!");

  //for loop

  for n in 1..11
  {
      println!("The square of {}  is {}",n,n*n);
  }
}