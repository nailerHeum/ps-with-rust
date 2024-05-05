// leetcode 443. String Compression
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input: Vec<char> = input.trim().chars().collect();
    let result = Solution::compress(&mut input);
    print!("{result}");
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut tmp_char = ' ';
        let mut count = 0;
        let mut is_single = true;
        let mut result: String = String::new();
        for c in chars.iter() {
            if *c == tmp_char {
                is_single = false;
                count += 1;
                continue;
            }

            if is_single == false {
                result.push_str(&count.to_string());
            }

            result.push_str(&c.to_string());
            count = 1;
            tmp_char = *c;
            is_single = true;
        }

        if count > 1 {
            result.push_str(&count.to_string());
        }

        for (idx, c) in result.chars().enumerate() {
            chars[idx] = c;
        }

        chars.truncate(result.len());
        return result.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
            6,
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::compress(&mut vec!['a']), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ]),
            4
        )
    }
}
