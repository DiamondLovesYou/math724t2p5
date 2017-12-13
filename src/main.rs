
// A rather brute force approach.

pub fn main() {
  let mut count = 1u64;

  for i in 0..6u64 {
    for j in 0..17u64 {
      if 493 < 87 * i + 29 * j { break; }
      count += (493 - 87 * i - 29 * j) / 6 + 1;
    }
  }

  println!("Harry Potter change (Test #2): count = {}", count);

  let mut count = 0u64;

  for i in 1..10u64 {
    for j in 1..10u64 {
      if i * 25 + j * 10 > 300 { break; }
      for k in 1..10u64 {
        let amount = i * 25 + j * 10 + k * 5;
        if amount > 300 { break; }

        if 300 - amount > 10 { continue; }

        count += 1;
      }
    }
  }

  println!("Three dollar bill change (Final): count = {}", count);
}
