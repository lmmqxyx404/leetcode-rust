/**
 * [2138] Divide a String Into Groups of Size k
 *
 * A string s can be partitioned into groups of size k using the following procedure:
 *
 * 	The first group consists of the first k characters of the string, the second group consists of the next k characters of the string, and so on. Each character can be a part of exactly one group.
 * 	For the last group, if the string does not have k characters remaining, a character fill is used to complete the group.
 *
 * Note that the partition is done so that after removing the fill character from the last group (if it exists) and concatenating all the groups in order, the resultant string should be s.
 * Given the string s, the size of each group k and the character fill, return a string array denoting the composition of every group s has been divided into, using the above procedure.
 *  
 * Example 1:
 *
 * Input: s = "abcdefghi", k = 3, fill = "x"
 * Output: ["abc","def","ghi"]
 * Explanation:
 * The first 3 characters "abc" form the first group.
 * The next 3 characters "def" form the second group.
 * The last 3 characters "ghi" form the third group.
 * Since all groups can be completely filled by characters from the string, we do not need to use fill.
 * Thus, the groups formed are "abc", "def", and "ghi".
 *
 * Example 2:
 *
 * Input: s = "abcdefghij", k = 3, fill = "x"
 * Output: ["abc","def","ghi","jxx"]
 * Explanation:
 * Similar to the previous example, we are forming the first three groups "abc", "def", and "ghi".
 * For the last group, we can only use the character 'j' from the string. To complete this group, we add 'x' twice.
 * Thus, the 4 groups formed are "abc", "def", "ghi", and "jxx".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists of lowercase English letters only.
 * 	1 <= k <= 100
 * 	fill is a lowercase English letter.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
// discuss: https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut s = s.clone();
        let sl = s.len();
        if sl % k as usize > 0 {
            for _ in 0..(k - sl as i32 % k) {
                s = s + &fill.to_string();
            }
        }
        let mut res = vec![];
        let mut i = 0;
        loop {
            let tmp = &s[i..i + k as usize];
            res.push(String::from(tmp));
            i = i + k as usize;
            if i >= s.len() {
                return res;
            }
        }
    }
}

// submission codes end

use std::vec;

use futures::channel::oneshot::channel;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2138() {
        assert_eq!(
            Solution::divide_string(String::from("hello wor"), 3, 'x'),
            vec![
                String::from("hel"),
                String::from("lo "),
                String::from("wor")
            ]
        );
    }
}