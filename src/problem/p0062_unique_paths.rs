/**
 * [62] Unique Paths
 *
 * There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.
 * Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.
 * The test cases are generated so that the answer will be less than or equal to 2 * 10^9.
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" />
 * Input: m = 3, n = 7
 * Output: 28
 *
 * Example 2:
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Down -> Down
 * 2. Down -> Down -> Right
 * 3. Down -> Right -> Down
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 100
 *
 */
//  Math Dynamic Programming Combinatorics
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths/
// discuss: https://leetcode.com/problems/unique-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; 110];110];
        for i in 2..=m {
            for j in 2..=n {
                dp[i as usize][j as usize] =
                    dp[(i - 1) as usize][j as usize] + dp[i as usize][(j - 1) as usize];
            }
        }
        return dp[m as usize][n as usize];
        /*
        if (m == 1 || n == 1) {
            1
        } else {
            Solution::unique_paths(m - 1, n) + Solution::unique_paths(m, n - 1)
        }
        */
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
