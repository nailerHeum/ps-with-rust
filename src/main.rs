// leetcode 605. Can Place Flowers
fn main() {
    Solution::can_place_flowers(vec![1, 0, 1], 1);
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut capability = 0;

        let mut zero_count = 1;
        for flower in flowerbed {
            if flower == 1 {
                capability += (zero_count - 1) / 2;
                zero_count = 0;
            } else {
                zero_count += 1;
            }
        }

        if zero_count != 0 {
            capability += zero_count / 2;
        }
        return capability >= n;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }
}
