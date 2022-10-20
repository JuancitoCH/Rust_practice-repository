// https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true&h_r=next-challenge&h_v=zen
pub fn timeConversion(s: &str) -> String {

    let hour = &s[..2];
    let time_day = &s[8..];
    let mut hour_formated_slice = String::new();
    let mut parser_hour:u8 = hour.parse().expect("The Conversion Failed");

    if time_day == "PM"{
        if  parser_hour > 0 && parser_hour < 12 {
            parser_hour+=12;
        }
    }else{
        if parser_hour == 12 {
            parser_hour=00
        }
    }

    hour_formated_slice.push_str(&format!("{:0>2}",parser_hour)[..]);
    hour_formated_slice.push_str(&s[2..8]);
    println!("{}",hour_formated_slice);
    format!("{}",hour_formated_slice)
}