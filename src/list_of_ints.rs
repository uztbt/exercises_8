pub fn mean(v: &Vec<i32>) -> f32 {
  if v.len() == 0 {return 0.0;}
  let mut sum: i32 = 0;
  for item in v {
    sum += item;
  }
  sum as f32 / v.len() as f32
}

pub fn median(v: &Vec<i32>) -> i32 {
  if v.len() == 0 { return 0;}
  let mut copy_vector = v.to_vec();
  copy_vector.sort();
  copy_vector[copy_vector.len()/2]
}
use std::collections::HashMap;
pub fn mode(v: &Vec<i32>) -> i32 {
  if v.len() == 0 { return 0; }
  let mut map: HashMap<i32, u8> = HashMap::new();
  for item in v {
    let count = map.entry(*item).or_insert(0);
    *count += 1;
  }
  let mut most_frequent_number_pair = (0, 0);
  for pair in map {
    if pair.1 > most_frequent_number_pair.1 {
      most_frequent_number_pair = pair
    }
  }
  most_frequent_number_pair.0
}