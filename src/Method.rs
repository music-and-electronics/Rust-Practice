struct Alien {
  health: u32,
  damage: u32
}

impl Alien {
  fn new(mut h: u32, d: u32) ->Alien 
  {
    if h> 100 { h = 100; }
    return Alien {health: h, damage: d};
  }
  
  fn warn() -> &'static str
  {
    return "Leave this planet";
  }

  fn attack(&self)
  {
    println!("Attack!");
  }

  fn suffer(&mut self, damage_from_other: u32)
  {
    self.health -= damage_from_other;
  }
}


pub fn run()
{
  let mut bork = Alien{ health: 100, damage: 5 };
  let mut berserk = Alien::new(150,15);
  println!("Berserk's health is {}",berserk.health);
  println!("{}",Alien::warn());
  berserk.attack();
  println!("Berserk's health is {}",berserk.health);
  berserk.suffer(31);
  println!("Berserk's health is {}",berserk.health);
}