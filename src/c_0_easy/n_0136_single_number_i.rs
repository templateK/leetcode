/* Created by templatek 2020-08-07
*
* Url:
*   https://leetcode.com/problems/single-number-i/
*
* Category:
*
*
* Question:
*   Given an array of numbers nums, in which exactly two elements appear only once
*   and all the other elements appear exactly twice.
*   Find the two elements that appear only once.
*/

fn single_number(arr: Vec<i32>) -> i32 {
    let mut s = 0;
    for i in arr {
        s ^= i;
    }
    s
}

#[cfg(test)]
mod tests {

    use crate::c_0_easy::n_0136_single_number_i::*;

    fn case0() {
        let input = vec![1];
        let result = single_number(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn case1() {
        let input = vec![2, 2, 1];
        let result = single_number(input);
        assert_eq!(result, 1);
    }
    #[test]
    fn case2() {
        let input = vec![4, 1, 2, 1, 2];
        let result = single_number(input);
        assert_eq!(result, 4);
    }
}
