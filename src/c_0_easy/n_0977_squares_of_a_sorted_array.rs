/* Created by templatek 2020-08-06
*
* Url:
*   https://leetcode.com/problems/squares-of-a-sorted-array/
*
* Category:
*
*
* Question:
*   Given an array of integers A sorted in non-decreasing order, return an array
*   of the squares of each number, also in sorted non-decreasing order.
*
*/

fn sorted_squares(_a: Vec<i32>) -> Vec<i32> {
    let iter = _a.iter().enumerate();
    let m = iter.min_by(|(_, a), (_, b)| (*a * *a).cmp(&(*b * *b)));
    match m {
        None => vec![],
        Some((ix, _)) => {
            let (left, right) = _a.split_at(ix);
            let mut l_iter = left.iter().rev();
            let mut r_iter = right.iter();
            let mut result = vec![];
            let mut l = l_iter.next();
            let mut r = r_iter.next();
            'exit: loop {
                match (l, r) {
                    (Some(ll), Some(rr)) => {
                        let lv = ll * ll;
                        let rv = rr * rr;
                        println!("both {}, {}", lv, rv);
                        if lv < rv {
                            l = l_iter.next();
                            result.push(lv);
                        } else if rv < lv {
                            r = r_iter.next();
                            result.push(rv);
                        } else {
                            l = l_iter.next();
                            r = r_iter.next();
                            result.push(lv);
                            result.push(rv);
                        }
                    }
                    (Some(ll), None) => {
                        println!("left {},", ll * ll);
                        result.push(ll * ll);
                        l = l_iter.next();
                    }
                    (None, Some(rr)) => {
                        println!("right {},", rr * rr);
                        result.push(rr * rr);
                        r = r_iter.next();
                    }
                    _ => {
                        break 'exit;
                    }
                }
            }
            result
        }
    }
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
        let input = vec![-3, -1, 2];
        let result = sorted_squares(input);
        assert_eq!(result, vec![1, 4, 9]);
    }

    #[test]
    fn case2() {
        let input = vec![-4, -1, 0, 3, 10];
        println!("input: {:?}", input);
        let result = sorted_squares(input);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }
}
