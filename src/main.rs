impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num) in nums.iter().enumerate() {
            let subtraction = target - num;
            let found = nums
                .iter()
                .enumerate()
                .position(|(current_position, current_element)| {
                    current_position != i && *current_element == subtraction
                });

            if let Option::Some(found_value) = found {
                return vec![i as i32, found_value as i32];
            }
        }

        vec![]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 3], 9), vec![]);
    }
}

fn main() {}
