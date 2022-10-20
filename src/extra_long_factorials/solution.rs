// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let input: u32 = input.trim().parse().unwrap();
    
//     let temp = factorial(input);
//     let result = format(temp);
    
//     println!("{}", result);
// }

// fn factorial(n: u32) -> Vec<u32>{
//     let mut result: Vec<u32> = vec![1];
//     for i in (1..n+1).rev() {
//         result = multiply(result, i);
//     }
//     return result;
// }

// fn multiply(a: Vec<u32>, b: u32) -> Vec<u32> {
// 	let mut result: Vec<u32> = vec![];
// 	let mut total: u32;
// 	let mut carry: u32 = 0;

// 	for i in 0..a.len() {
// 		total = a[i] * b + carry;
// 		result.push(total % 1000);
// 		carry = total / 1000;
// 	}
//     if carry != 0 {
// 	   result.push(carry);
//     }
	
// 	return result;
// }

// fn format(v: Vec<u32>) -> String{
// 	let mut result = String::new();
//     let mut temp = String::new();
    
//     for i in (0..v.len()).rev() {
//         let n = v[i].to_string();
        
//         if v[i] >= 100 || i == v.len() - 1 {
//             temp = n;
//         } else if v[i] < 100 && v[i] >= 10 {
//             let zero = "0".to_string();
//             temp = zero + &n;
//         } else if v[i] < 10 {
//             let zero = "00".to_string();
//             temp = zero + &n;
//         }
//         result = result + &temp;
// 	}
		
// 	return result;
// }


// //  second implementation

// use std::io;

// const BOUNDARY :u64 = 10000000000000000u64;

// fn mul(longnum : &mut Vec<u64>, multiplier : u8) {
//     let mut carryover : u64 = 0;
//     let n = longnum.len();

//     for i in 1..(n+1) {
//         longnum[n-i] *= multiplier as u64;
//         longnum[n-i] += carryover;

//         carryover = longnum[n-i] / BOUNDARY;
//         longnum[n-i] = longnum[n-i] % BOUNDARY;
//     }

//     if carryover > 0 {
//         longnum.insert(0, carryover);
//     }
// }

// fn output(longnum : Vec<u64>) {
//     print!("{}", longnum[0]);
//     for i in 1..longnum.len() {
//         print!("{:0>16}", longnum[i])
//     }
//     println!("");
// }

// fn main() {
//     let mut buffer = String::new();

//     io::stdin().read_line(&mut buffer).ok().expect("read error");
//     let n : u8 = buffer.trim().parse().ok().expect("parse error");

//     let mut result : Vec<u64> = vec![1 as u64];

//     for i in 1..(n+1) {
//         mul(&mut result, i);
//     }

//     output(result);
// }