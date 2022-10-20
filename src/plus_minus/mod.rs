// Given an array of integers, calculate the ratios of its elements that are positive, negative, and zero. Print the decimal value of each fraction on a new line with  places after the decimal.

// Note: This challenge introduces precision problems. The test cases are scaled to six decimal places, though answers with absolute error of up to 10^-4 are acceptable.

pub fn plus_minus(arr: &[i32]) {

    let mut positive:f32 = 0.0;
    let mut negative:f32 = 0.0;
    let mut zero:f32 = 0.0;
    for &number in arr{
        if number>0 {
            positive +=1.0;
        }
        else if number ==0 {
            zero+=1.0;
        }
        else{
            negative+=1.0;
        }
    }
    println!("{:.6}",positive/arr.len() as f32);
    println!("{:.6}",negative/arr.len() as f32);
    println!("{:.6}",zero/arr.len() as f32);
}
