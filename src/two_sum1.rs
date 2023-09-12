pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
