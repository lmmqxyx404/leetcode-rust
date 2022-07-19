use core::num;

/**
 * [118] Pascal's Triangle
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *  
 * Constraints:
 *
 * 	1 <= numRows <= 30
 *
 */
//  Array Dynamic Programming
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle/
// discuss: https://leetcode.com/problems/pascals-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut a: Vec<Vec<i32>> = vec![];
        let mut b = vec![];
        for i in 0..num_rows {
            b.clear();
            for j in 0..=i {
                if (j == 0 || j == i) {
                    b.push(1);
                } else {
                    b.push(a[(i - 1) as usize][(j - 1) as usize] + a[(i - 1) as usize][j as usize]);
                }
            }
            a.push(b.clone());
        }
        a
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_118() {
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
    }
}
