// baekjoon 10926 ??!
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    let result = Solution::being_surprised(input);
    print!("{result}");
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn being_surprised(input: &str) -> String {
        format!("{input}??!")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::being_surprised("joonas"), "joonas??!".to_string());
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::being_surprised("baekjoon"),
            "baekjoon??!".to_string()
        );
    }
}
