pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<&T>> {
    let num_cols = matrix.get(0).unwrap().len();
    let num_rows = matrix.len();
    let mut transposed: Vec<Vec<&T>> = vec![Vec::with_capacity(num_rows); num_cols];

    for row in matrix {
        for (i, element) in row.iter().enumerate() {
            transposed[i].push(element);
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

        let expected = vec![
            vec![&input[0][0], &input[1][0]],
            vec![&input[0][1], &input[1][1]],
            vec![&input[0][2], &input[1][2]],
        ];

        let result = transpose(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_with_heap_allocated_objects() {
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        let d = String::from("d");
        let e = String::from("e");
        let f = String::from("f");

        let input = vec![vec![a, b, c], vec![d, e, f]];

        let expected = vec![
            vec![&input[0][0], &input[1][0]],
            vec![&input[0][1], &input[1][1]],
            vec![&input[0][2], &input[1][2]],
        ];

        let result = transpose(&input);

        assert_eq!(result, expected);
    }
}
