/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *  
 * Example 1:
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * Example 2:
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *  
 * Constraints:
 *
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let is_even = (m + n) % 2;
        let limit = (m + n) / 2;
        let (mut i, mut j, mut k, mut total) = (0, 0, 0, 0.0);
        if m == 0 && n == 0 {
            return 0.0;
        }
        loop {
            let mut tmp: f64 = 0.0;
            if j == m {
                tmp = nums2[k] as f64;
                k = k + 1;
            } else if k == n {
                tmp = nums1[j] as f64;
                j = j + 1;
            } else {
                if nums1[j] > nums2[k] {
                    tmp = nums2[k] as f64;
                    k += 1;
                } else {
                    tmp = nums1[j] as f64;
                    j += 1;
                }
            }
            if limit == 0 || i >= limit - 1 {
                total += tmp as f64;
                if i == limit {
                    if is_even != 0 as usize {
                        return tmp;
                    }
                    return total / 2.0;
                }
            }
            i += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        let nums1: Vec<i32> = vec![3];
        let nums2: Vec<i32> = vec![4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 3.0);
    }
}
