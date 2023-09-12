impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (i, v) in nums.iter().enumerate() {
            match map.get(&(target - *v)) {
                Some(&p) => return vec![p as i32, i as i32],
                None => {
                    map.insert(*v, i);
                }
            }
        }

        panic!("No solution found!");
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![5, 1, 7, 3], 10), vec![2, 3]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![7, 7, 7, 7, 7, 7], 14), vec![0, 1]);
    }

    #[test]
    #[should_panic(expected = "No solution found!")]
    fn panic_when_no_solution_found() {
        Solution::two_sum(vec![1, 2], 14);
    }
}

fn main() {}
