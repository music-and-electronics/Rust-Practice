pub fn run()
{
  string_prac();
}

fn string_prac()
{
  let magician1 = "Merlin";
  let magician2: &'static str = "Gandalf";
  let greeting = "Hello 세계";
  println!("Magicain {} greets magician {} with {}"
                      , magician1, magician2,greeting);
  let mut str1 = String::new();
  str1.push_str("Hello World!");
  
  println!("{}",str1);

  for c in str1.chars()
  {
    print!("{} - ",c);
  }

  println!("The size is {}",how_long(&str1))
}

fn how_long(s: &str)->usize {s.len()}