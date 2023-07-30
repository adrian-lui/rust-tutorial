fn main() {
  println!("{:?}", string_to_array("Robin Singh"));
  println!("{:?}", string_to_array("CodeWars"));
  println!("{:?}", string_to_array("I love arrays they are my favorite"));
  println!("{:?}", string_to_array("1 2 3"));
}

fn string_to_array(s: &str) -> Vec<String> {
   s.split(' ').collect::<Vec<&str>>().iter().map(|word| word.to_string()).collect::<Vec<String>>()
}