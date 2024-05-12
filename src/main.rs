// leetcode 283. Move Zeroes

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;
        let mut scan_to = nums.len();
        while idx < scan_to {
            if nums[idx] == 0 {
                nums.remove(idx);
                nums.push(0);
                scan_to -= 1;
            } else {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut case1 = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut case1);
        assert_eq!(case1, [1, 3, 12, 0, 0])
    }

    #[test]
    fn case_2() {}

    #[test]
    fn case_3() {}
}
