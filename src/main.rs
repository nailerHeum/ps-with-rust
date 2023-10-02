// leetcode 345. Reverse Vowels of a String
fn main() {
    Solution::reverse_vowels("asdf".into());
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: Vec<char> = vec!['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
        let mut collected_vowels: Vec<char> = s
            .chars()
            .filter(|c| vowels.iter().any(|x| x == c))
            .collect();

        s.chars()
            .map(|c| {
                if vowels.iter().any(|v| *v == c) {
                    collected_vowels.pop().unwrap()
                } else {
                    c
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_vowels("hello".into()),
            String::from("holle")
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".into()),
            String::from("leotcede")
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::reverse_vowels("aA".into()), String::from("Aa"));
    }
}
