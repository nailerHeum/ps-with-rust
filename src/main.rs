// leetcode 151. Reverse Words in a String
fn main() {
    Solution::reverse_words("asdf".into());
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".into()),
            String::from("blue is sky the")
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".into()),
            String::from("world hello")
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".into()),
            String::from("example good a")
        );
    }
}
