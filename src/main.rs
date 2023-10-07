// leetcode 238. Product of Array Except Self
fn main() {
    Solution::product_except_self(vec![1, 2, 3, 4]);
}

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut acc = 1;
        let mut acc_rev = 1;

        nums.iter()
            .map(|x| {
                let prev_product = acc.clone();
                acc = acc * x;
                return prev_product;
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, x)| {
                let prev_product = acc_rev.clone();
                acc_rev = acc_rev * nums[nums.len() - i - 1];
                return prev_product * x;
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
