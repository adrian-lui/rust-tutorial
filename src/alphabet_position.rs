fn main() {
  println!("{}", alphabet_position("The sunset sets at twelve o' clock."));
  println!("{}", alphabet_position("The narwhal bacons at midnight."));
}

fn alphabet_position(text: &str) -> String {
  let mut ret = String::new();
  for c in text.chars() {
    if c.is_ascii_alphabetic() {
      ret.push_str(format!("{} ",(c.to_ascii_uppercase() as u8) - 64).as_str());
    } 
  }
  ret.trim_end().to_string()
}