// leetcode 334. Increasing Triplet Subsequence
fn main() {
    Solution::increasing_triplet(vec![1, 2, 3, 4]);
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min_midium = i32::MAX;
        let mut min_small = i32::MAX;

        nums.into_iter().any(|x| {
            if min_small >= x {
                min_small = x;
                false
            } else if min_small < x && x <= min_midium {
                min_midium = x;
                false
            } else {
                true
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }
}
