pub fn run() {

}

struct Alien {
  health: u32,
  damage: u32,
}

struct Zombie {
  health: u32,
  damage: u32,
}

struct Predetor {
  health: u32,
  damage: u32,
}

trait Monster {
  fn new(health: u32, damage: u32) -> Self;
  fn attack(&self);
  fn noise(&self) -> &'static str;
  fn attacks_with_sound(&self) {
      println!("The Monster attacks by making an awkward sound {}"
      , self.noise());
  }
}