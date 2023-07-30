fn main() {
  println!("{}", number(&[(10,0),(3,5),(5,8)]));
  println!("{}", number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]));
  println!("{}", number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]));
}

fn number(bus_stops:&[(i32,i32)]) -> i32 {
  let mut remaining = 0;
  for stop in bus_stops.iter() {
    remaining += stop.0;
    remaining -= stop.1;
  }
  remaining
}
