/**
 * [858] Mirror Reflection
 *
 * There is a special square room with mirrors on each of the four walls. Except for the southwest corner, there are receptors on each of the remaining corners, numbered 0, 1, and 2.
 * The square room has walls of length p and a laser ray from the southwest corner first meets the east wall at a distance q from the 0^th receptor.
 * Given the two integers p and q, return the number of the receptor that the ray meets first.
 * The test cases are guaranteed so that the ray will meet a receptor eventually.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/18/reflection.png" style="width: 218px; height: 217px;" />
 * Input: p = 2, q = 1
 * Output: 2
 * Explanation: The ray meets receptor 2 the first time it gets reflected back to the left wall.
 *
 * Example 2:
 *
 * Input: p = 3, q = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= q <= p <= 1000
 *
 */
//  Math Geometry
pub struct Solution {}

// problem: https://leetcode.com/problems/mirror-reflection/
// discuss: https://leetcode.com/problems/mirror-reflection/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn gcd(p: i32, q: i32) -> i32 {
        if q == 0 {
            p
        } else {
            Solution::gcd(q, p % q)
        }
    }
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let times = p / Solution::gcd(p, q);
        if times % 2 == 0 {
            2
        } else {
            if times * q / p % 2 == 1 {
                1
            } else {
                0
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_858() {
        assert_eq!(Solution::mirror_reflection(3, 1), 1);
    }
}
