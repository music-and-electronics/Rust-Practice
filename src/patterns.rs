pub fn run()
{
  magic();
}

fn magic() 
{
  let magical_num : i32 = 11;
  match magical_num 
  {
    1          => println!("One"),
    2|3|5|7|11 => println!("Prime!"),
    40...42    => println!("In range"),
    _          => println!("No shit!"),
  }
}

fn loki()
{
  let loki = ("Loki",true,800u32);
  match loki
  {
    (name,demi, _) if demi =>
    {
      print!("This is a demigod ");
      println!("called {}",name);
    },
    (name, _ , _) if name == "Thor" =>
    {
      println!("This is thor!");
    },
    (_, _, pow) if pow <= 100 =>
    {
      println!("This is a powerless god");
    }
    _ => 
    { 
      println!("This is something else");
    }
  }
}