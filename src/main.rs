use std::ops::Range;
fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut mp = std::collections::HashMap::<i32, i32>::new();
    for (idx, &num) in nums.iter().enumerate() {
        if let Some(&i) = mp.get(&(target - num)) {
            return vec![idx as i32, i];
        }
        mp.insert(num, idx as i32);
    }
    unreachable!()
}
