fn main() {
  println!("{}", bool_to_word(true));
  println!("{}", bool_to_word(false));
}

fn bool_to_word(value: bool) -> &'static str {
  if value {
    "Yes"
  } else {
    "No"
  }
}