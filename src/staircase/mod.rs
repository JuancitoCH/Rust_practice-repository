pub fn staircase(n: i32) {
    for x in 0..n {
        let mut print_line = String::from("");
        for spaces in (0..n).rev(){
            if spaces <= x {
                print_line = format!("{}{}",print_line,"#");
            }else{
                print_line = format!("{}{}",print_line," ");
            }
        }
        println!("{}",print_line)
    }
}