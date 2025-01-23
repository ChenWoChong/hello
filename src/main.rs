pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut idx = (m + n - 1) as usize;
    let mut left = (m - 1) as usize;
    let mut right = (n - 1) as usize;
    while right < n as usize {
        if left < m as usize && nums1[left] >= nums2[right] {
            nums1[idx] = nums1[left];
            left -= 1;
        } else {
            nums1[idx] = nums2[right];
            right -= 1;
        }
        idx -= 1;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut n1 = vec![1, 2, 3, 4, 0, 0, 0];
        let mut n2 = vec![5, 6, 7];
        let m: i32 = 4;
        let n: i32 = 3;
        merge(&mut n1, m, &mut n2, n);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], n1);
    }
}
