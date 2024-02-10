fn main() {
    println!("Hello, world!");
}

trait Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool;
}

struct Impl1;
impl Solution for Impl1 {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        assert!(m > 0);
        let n = matrix[0].len();
        assert!(n > 0);
        let mut i = 0;
        let mut j = n;
        while i < m && j >= 1 {
            if matrix[i][j - 1] == target {
                return true;
            } else if matrix[i][j - 1] > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_impl1() {
        let m1: Vec<Vec<i32>> = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert_eq!(Impl1::search_matrix(m1.clone(), 5), true);
        assert_eq!(Impl1::search_matrix(m1, 20), false);
    }
}
