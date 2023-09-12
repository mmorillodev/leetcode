use std::{
    collections::{hash_map::Entry, HashMap},
    vec,
};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexed = Self::create_position_indexes(&nums);

        for (i, num) in nums.iter().enumerate() {
            let missing = target - num;

            if let Entry::Occupied(e) = indexed.entry(missing) {
                if let Some(right_position) = e.get().iter().find(|curr_pos| **curr_pos != i as i32)
                {
                    return vec![i as i32, *right_position];
                }
            }
        }

        vec![]
    }

    fn create_position_indexes(nums: &Vec<i32>) -> HashMap<i32, Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for (i, current_num) in nums.iter().enumerate() {
            match map.entry(*current_num) {
                Entry::Vacant(e) => {
                    e.insert(vec![i as i32]);
                }
                Entry::Occupied(mut e) => {
                    let positions = e.get_mut();
                    positions.push(i as i32);
                }
            }
        }

        return map;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_map_for_each_element_with_their_positions() {
        let vector = vec![2, 7, 7, 11, 15];
        let map = Solution::create_position_indexes(&vector);

        assert_eq!(map.get(&2), Option::Some(vec![0]).as_ref());
        assert_eq!(map.get(&7), Option::Some(vec![1, 2]).as_ref());
        assert_eq!(map.get(&11), Option::Some(vec![3]).as_ref());
        assert_eq!(map.get(&15), Option::Some(vec![4]).as_ref());
    }

    #[test]
    fn return_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![5, 1, 7, 3], 10), vec![2, 3]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![7, 7, 7, 7, 7, 7], 14), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 3], 9), vec![]);
    }
}

fn main() {}
