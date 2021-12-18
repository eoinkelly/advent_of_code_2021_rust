pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let num_cols = matrix.get(0).unwrap().len();
    let mut transposed: Vec<Vec<T>> = vec![vec![]; num_cols];

    for row in matrix {
        for (i, element) in row.iter().enumerate() {
            transposed[i].push(element.clone());
        }
    }

    transposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let input = vec![vec![1, 2, 4], vec![5, 6, 7]];

        let expected = vec![vec![1, 5], vec![2, 6], vec![4, 7]];
        let result = transpose(&input);
        assert_eq!(result, expected);
    }
}
