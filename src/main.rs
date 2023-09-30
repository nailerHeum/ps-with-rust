// leetcode 1431. Kids With the Greatest Number of Candies
fn main() {
    Solution::kids_with_candies(vec![1, 2, 3], 1);
}

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        return candies.iter().map(|x| x + extra_candies >= max).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
