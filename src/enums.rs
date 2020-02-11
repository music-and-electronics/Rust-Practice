
type species = &'static str;
#[derive(Debug)]
enum PlanetaryMonster {
  VenusMonster(species,i32),
  MarsMonster(species,i32)
}

use PlanetaryMonster::MarsMonster;

pub fn run() {

  let martian = MarsMonster("Chela",42);
  println!("{:?}",martian);
}