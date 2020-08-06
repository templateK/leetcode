/* Created by templatek 2020-08-06
*
* Url:
*   https://leetcode.com/problems/squares-of-a-sorted-array/
*
* Category:
*     Adhoc programming
*
* Question:
*   Given an array of integers A sorted in non-decreasing order, return an array
*   of the squares of each number, also in sorted non-decreasing order.
*
*/

fn sorted_squares(arr: Vec<i32>) -> Vec<i32> {
    let mut li = 0;
    let mut ri = arr.len().saturating_sub(1);
    let mut ix = arr.len().saturating_sub(1);

    // We store squared values from largest to smallest.
    // So, we need to allocate the memory ahead of the loop.
    let mut result = vec![0; arr.len()];

    for _ in 0..arr.len() {
        // case for li abs value is greater than ri abs value.
        if arr[li] + arr[ri] <= 0 {
            result[ix] = arr[li] * arr[li];
            li += 1;
        } else {
            result[ix] = arr[ri] * arr[ri];
            ri = ri.saturating_sub(1);
        }
        ix = ix.saturating_sub(1);
    }
    result
}

#[cfg(test)]
mod tests {

    use crate::c_0_easy::n_0977_squares_of_a_sorted_array::*;

    #[test]
    fn case0() {
        let input = vec![];
        let result = sorted_squares(input);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn case1() {
        let input = vec![1];
        let result = sorted_squares(input);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn case2() {
        let input = vec![-3, -1, 2];
        let result = sorted_squares(input);
        assert_eq!(result, vec![1, 4, 9]);
    }

    #[test]
    fn case3() {
        let input = vec![-4, -1, 0, 3, 10];
        println!("input: {:?}", input);
        let result = sorted_squares(input);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }
}
