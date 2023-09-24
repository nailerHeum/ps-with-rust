// leetcode 1768 Merge Strings Alternately
fn main() {
  Solution::merge_alternately(String::from("abc"), String::from("pqr"));
}

#[derive(Debug)]
struct Solution {}

impl Solution {
  pub fn merge_alternately(word1: String, word2: String) -> String {
    let larger_len = if word1.len() > word2.len() { word1.len() } else { word2.len() };
    let mut word1iter = word1.chars();
    let mut word2iter = word2.chars();
    let mut result: Vec<char> = Vec::new();
    for _ in 0..larger_len {
      match word1iter.next() {
        Some(v) => result.push(v),
        None => (),
      }
      match word2iter.next() {
        Some(v) => result.push(v),
        None => (),
      }
    }
    
    return result.iter().collect();
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn case_1() {
    assert_eq!(Solution::merge_alternately(String::from("abc"), String::from("pqr")), String::from("apbqcr"));
  }

  #[test]
  fn case_2() {
    assert_eq!(Solution::merge_alternately(String::from("ab"), String::from("pqrs")), String::from("apbqrs"));
  }

  #[test]
  fn case_3() {
    assert_eq!(Solution::merge_alternately(String::from("abcd"), String::from("pq")), String::from("apbqcd"));
  }
}
