use std::convert::TryInto;

/**
 * [1220] Count Vowels Permutation
 *
 * Given an integer n, your task is to count how many strings of length n can be formed under the following rules:
 *
 * 	Each character is a lower case vowel ('a', 'e', 'i', 'o', 'u')
 * 	Each vowel 'a' may only be followed by an 'e'.
 * 	Each vowel 'e' may only be followed by an 'a' or an 'i'.
 * 	Each vowel 'i' may not be followed by another 'i'.
 * 	Each vowel 'o' may only be followed by an 'i' or a 'u'.
 * 	Each vowel 'u' may only be followed by an 'a'.
 *
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 5
 * Explanation: All possible strings are: "a", "e", "i" , "o" and "u".
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: 10
 * Explanation: All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".
 *
 * Example 3:
 *
 * Input: n = 5
 * Output: 68
 *  
 * Constraints:
 *
 * 	1 <= n <= 2 * 10^4
 *
 */
//  Dynamic Programming
pub struct Solution {}

// problem: https://leetcode.com/problems/count-vowels-permutation/
// discuss: https://leetcode.com/problems/count-vowels-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp = vec![vec![1; 6]; 20048];
        let num = 1000000007;
        for i in 2..=n {
            let i: usize = i as usize;
            dp[i][1] = (dp[i - 1][2]) % num;
            dp[i][2] = (dp[i - 1][1] + dp[i - 1][3]) % num;
            dp[i][3] =
                ((dp[i - 1][1] + dp[i - 1][2]) % num + (dp[i - 1][4] + dp[i - 1][5]) % num) % num;
            dp[i][4] = (dp[i - 1][3] + dp[i - 1][5]) % num;
            dp[i][5] = (dp[i - 1][1]) % num;
        }
        let mut ans = 0;
        for i in 1..6 {
            ans = (ans + dp[n as usize][i as usize]) % num;
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1220() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
        assert_eq!(Solution::count_vowel_permutation(2), 10);
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
}
