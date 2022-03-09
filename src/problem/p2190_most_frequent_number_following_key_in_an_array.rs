/**
 * [2190] Most Frequent Number Following Key In an Array
 *
 * You are given a 0-indexed integer array nums. You are also given an integer key, which is present in nums.
 * For every unique integer target in nums, count the number of times target immediately follows an occurrence of key in nums. In other words, count the number of indices i such that:
 *
 * 	0 <= i <= nums.length - 2,
 * 	nums[i] == key and,
 * 	nums[i + 1] == target.
 *
 * Return the target with the maximum count. The test cases will be generated such that the target with maximum count is unique.
 *  
 * Example 1:
 *
 * Input: nums = [1,100,200,1,100], key = 1
 * Output: 100
 * Explanation: For target = 100, there are 2 occurrences at indices 1 and 4 which follow an occurrence of key.
 * No other integers follow an occurrence of key, so we return 100.
 *
 * Example 2:
 *
 * Input: nums = [2,2,2,2,3], key = 2
 * Output: 2
 * Explanation: For target = 2, there are 3 occurrences at indices 1, 2, and 3 which follow an occurrence of key.
 * For target = 3, there is only one occurrence at index 4 which follows an occurrence of key.
 * target = 2 has the maximum number of occurrences following an occurrence of key, so we return 2.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 1000
 * 	1 <= nums[i] <= 1000
 * 	The test cases will be generated such that the answer is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/
// discuss: https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if nums[i] as i32 == key && i + 1 < nums.len() {
                if let Some(x) = map.get_mut(&nums[i + 1]) {
                    *x = *x + 1;
                } else {
                    map.insert(nums[i + 1], 1);
                }
            }
        }
        let (mut maxn, mut ans) = (-1, 0);
        for (k, v) in map {
            //println!("key: {},value: {}", k, v);
            if v > maxn {
                maxn = v;
                ans = k;
            }
        }
        ans
    }
}

// submission codes end

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2190() {
        assert_eq!(Solution::most_frequent(vec![1, 100, 200, 1, 100], 1), 2);
    }
}
