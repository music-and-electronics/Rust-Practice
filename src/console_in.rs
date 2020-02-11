pub fn run()
{
  let mut ret = String::new();
  std::io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
  let num: i32 = ret.trim().parse().expect("Fail to parse");
  let result: String = sum(num).to_string();
  println!("num is {} ",result);
}
fn sum (num : i32) -> i32
{
  num + num
}

