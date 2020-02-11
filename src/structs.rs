struct Player
{
  nickname: &'static str,
  health: u32,
  level: u8
}
pub fn run()
{
  let mut pl1 = Player{ nickname:"Hi", health: 50 ,level: 2 };
  println!("Player {} is at level {} with point {}",pl1.nickname
                                                   ,pl1.level
                                                   ,pl1.health);
}