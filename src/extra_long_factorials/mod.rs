
pub fn extraLongFactorials(n: i32)-> String {
    let mut result:Vec<i32> = vec![];
    if n==1{
        return String::from('1');
    }
    for i in 1..n{
        if i==1{
            result = multiply_vec(
                number_to_array_of_units(n),
                number_to_array_of_units(n - (i as i32)),
            );
        }
        else if i>1{
            result = multiply_vec(
                result,
                number_to_array_of_units(n - (i as i32)),
            );
        };
    };
    // println!("Result : {:?}",&result);
    format_vector(&result)
}

fn format_vector(vector:&Vec<i32>)->String{
    let mut response = String::new();
    for i in (0..(vector.len())).rev(){
        response.push_str(&format!("{}",vector[i])[..]);
    }
    // println!("{}",response);
    response
}

fn number_to_array_of_units(n:i32)-> Vec<i32> {
    if n>9 {
        let mut number = n;        
        let mut vector:Vec<i32> =vec![];
        while number !=0 {   
            let division = number%10;
            number = number/10;
            // println!("{}",number);
            vector.push(division);
        }
        vector

    }else{
        vec![n]
    }

}

fn multiply_vec(vector1:Vec<i32>,vector2:Vec<i32>)->Vec<i32>{
    let mut vector_response:Vec<i32> = vec![];
    for (i,numvec1) in vector1.iter().enumerate(){
        for (j,numvec2) in vector2.iter().enumerate(){
            
            let mult = numvec1*numvec2;
            if mult>9{
                    while vector_response.len()< i+j+2{
                        vector_response.push(0);
                    }
                    vector_response[j+i]+=mult%10;
                    vector_response[j+i+1]+=mult/10;

                    if vector_response[j+i]> 9{
                        vector_response[j+i+1] += vector_response[j+i]/10;
                        vector_response[j+i] = vector_response[j+i]%10;
                    }
                    if vector_response[j+i+1] > 9{
                        while vector_response.len()< i+j+3{
                            vector_response.push(0);
                        }
                        vector_response[j+i+2] += vector_response[j+i+1]/10;
                        vector_response[j+i+1]=vector_response[j+i+1]%10;
                    }

                }
                else {
                    while vector_response.len()< i+j+1{
                        vector_response.push(0);
                    }
                    vector_response[j+i]+=mult;
                    if vector_response[j+i]> 9{
                        while vector_response.len()< i+j+2{
                            vector_response.push(0);
                        }
                        vector_response[j+i+1] += vector_response[j+i]/10;
                        vector_response[j+i] = vector_response[j+i]%10;
                    }
                }
            }
        }   
    println!(" mult_{:?}",vector_response);
    vector_response

}

// fn suma_vec(vector1:&mut Vec<i32>,vector2:Vec<i32>){
//     println!("{:?} + {:?}",vector1,vector2);
//     for (i,numvec) in vector2.iter().enumerate(){
//         vector1[i]+=numvec;
//         if vector1[i]>9{
//             while  vector1.len()<i+3{
//                 vector1.push(0)
//             }
//             vector1[i+1]+=vector1[i]/10;
//             vector1[i]=vector1[i]%10;
           
//         }
//         if vector1[i+1]>9{
//             while  vector1.len()<i+3{
//                 vector1.push(0)
//             }
//             vector1[i+2]+=vector1[i+1]/10;
//             vector1[i+1]=vector1[i+1]*10;
//         }
//     }   
//     println!("{:?}",vector1)
// }
                        // pub fn extraLongFactorials(n: i32)-> i128 {
                        //     let mut result: i128 = 1;
                        //     for i in 0..n{
                        //         result*=i128::from( n - (i as i32) );
                        //     }
                        //     println!("{}",result);
                            
                        //     // 15511210043330985984000000
                        //     result
                        // }
                        // I can't resolve it doing operations due the limitation of the numbers
                        // I have to desasembly the number and the asembly in a string, but i dont wanna do it so for later
                        //  or for see the solution of someone i copy the code in the file solution
