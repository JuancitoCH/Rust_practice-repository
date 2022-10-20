pub fn extraLongFactorials(n: i32)-> i128 {
    let mut result: i128 = 1;
    for i in 0..n{
        result*=i128::from( n - (i as i32) );
    }
    println!("{}",result);
    
    // 15511210043330985984000000
    result
}
// I can't resolve it doing operations due the limitation of the numbers
// I have to desasembly the number and the asembly in a string, but i dont wanna do it so for later
//  or for see the solution of someone i copy the code in the file solution

