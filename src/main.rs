
// A rather brute force approach.

pub fn main() {
  let mut count = 1u64;

  for i in 0..5u64 {
    for j in 0..17u64 {
      if 493 < 87 * i + 29 * j { break; }
      count += (493 - 87 * i - 29 * j) / 6;
    }
  }

  println!("count = {}", count);
}
