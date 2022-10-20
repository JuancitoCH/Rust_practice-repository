pub fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut left_diagonal:i32 = 0;
    let mut right_diagonal :i32= 0;
    // it supose to be square matrix
    for n in 0..arr.len(){
        left_diagonal += arr[n][n];
        right_diagonal += arr[n][arr.len()-1-n];
    }
    (left_diagonal - right_diagonal).abs()

}
