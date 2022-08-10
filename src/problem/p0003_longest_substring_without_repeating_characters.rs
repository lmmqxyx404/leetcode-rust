use std::collections::HashMap;

/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *  
 * Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
//  Hash Table String Sliding Window
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // let str: Vec<char> = s.chars().collect();
        let (mut left, mut ans, mut right) = (0, 0, 0);
        let mut mp: HashMap<char, i32> = HashMap::new();
        for i in s.chars() {
            if let Some(&x) = mp.get(&i) {
                if left > x {
                    ans = ans.max(right - left + 1);
                } else {
                    left = x + 1;
                }
            } else {
                ans = ans.max(right - left + 1);
            }
            mp.insert(i, right);
            right += 1;
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbbbb")),
            1
        );
    }
}
