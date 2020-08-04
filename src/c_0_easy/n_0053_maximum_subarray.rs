/* Created by templatek 2020-08-04
 *
 * Url:
 *   https://leetcode.com/problems/maximum-subarray/
 *
 * Category:
 *   Dynamic programming, Adhoc programming
 *
 * Question:
 *   Given an integer array nums, find the contiguous subarray (containing at least one number)
 *   which has the largest sum and return its sum.
 *
 */

fn maximum_subsequence_sum(nums: Vec<i32>) -> i32 {
    let mut streak = nums[0];
    let mut global = streak;
    // calcuate maximum local streak as long as possible
    // and then compare it with global streak
    for i in 1..nums.len() {
        if nums[i] + streak > nums[i] {
            streak = nums[i] + streak;
        } else {
            streak = nums[i];
        }
        global = std::cmp::max(streak, global);
    }
    global
}

#[cfg(test)]
mod tests {
    use crate::easy::n_0053_maximum_subarray::*;

    #[test]
    fn case0() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = maximum_subsequence_sum(input);
        assert_eq!(result, 55);
    }

    #[test]
    fn case1() {
        let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = maximum_subsequence_sum(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn case2() {
        let input = vec![-1];
        let result = maximum_subsequence_sum(input);
        assert_eq!(result, -1);
    }

    #[test]
    fn case3() {
        let input = vec![-2, -1];
        let result = maximum_subsequence_sum(input);
        assert_eq!(result, -1);
    }
}
