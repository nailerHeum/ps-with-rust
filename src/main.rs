// leetcode 1768 Merge Strings Alternately
fn main() {
    Solution::gcd_of_strings(String::from("abc"), String::from("pqr"));
}

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (shorter_length, shorter_string, longer_length, longer_string) =
            if str1.len() < str2.len() {
                (str1.len(), str1, str2.len(), str2)
            } else {
                (str2.len(), str2, str1.len(), str1)
            };

        for i in 1..=shorter_length {
            let short_rem = shorter_length.checked_rem(i).unwrap();
            let short_div = shorter_length.checked_div(i).unwrap();
            let long_rem = longer_length.checked_rem(short_div).unwrap();
            if short_rem > 0 || long_rem > 0 {
                continue;
            }

            let gcd_str = &shorter_string[0..short_div];
            let multiply = longer_length.checked_div(short_div).unwrap();
            if gcd_str.repeat(i) == shorter_string && gcd_str.repeat(multiply) == longer_string {
                return String::from(gcd_str);
            }
        }

        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn case_1() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("ABC"), String::from("ABCABC")),
            String::from("ABC")
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
            String::from("AB")
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("LEET"), String::from("CODE")),
            String::from("")
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("AAAAAAAAA"), String::from("AAACCC")),
            String::from("aa")
        );
    }
}
