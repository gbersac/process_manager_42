use matrix::Matrix;

#[test]
fn test_get_col() {
    let mat = Matrix::from_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
    let expected = vec![1, 4];
    let result = mat.get_col(0);
    assert!(result == expected);
    let expected = vec![3, 6];
    let result = mat.get_col(2);
    println!("{:?}", result);
    assert!(result == expected);
}
