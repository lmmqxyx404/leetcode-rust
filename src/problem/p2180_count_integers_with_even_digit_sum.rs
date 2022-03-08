/**
 * [2180] Count Integers With Even Digit Sum
 *
 * Given a positive integer num, return the number of positive integers less than or equal to num whose digit sums are even.
 * The digit sum of a positive integer is the sum of all its digits.
 *  
 * Example 1:
 *
 * Input: num = 4
 * Output: 2
 * Explanation:
 * The only integers less than or equal to 4 whose digit sums are even are 2 and 4.    
 *
 * Example 2:
 *
 * Input: num = 30
 * Output: 14
 * Explanation:
 * The 14 integers less than or equal to 30 whose digit sums are even are
 * 2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26, and 28.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-integers-with-even-digit-sum/
// discuss: https://leetcode.com/problems/count-integers-with-even-digit-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut res = 0;
        for i in 1..=num {
            if Self::judge(i) {
                res += 1;
            }
        }
        res
    }
    fn judge(mut i: i32) -> bool {
        let mut res = 0;
        while i != 0 {
            res += i % 10;
            i = i / 10;
        }
        res % 2 == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2180() {
        assert_eq!(Solution::count_even(30), 14 );
    }
}
