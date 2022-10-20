pub fn minimaxSum(arr: &[i32]) {
    let mut minimum = i64::MAX;
    let mut maximun = i64::MIN;
    for (i,element) in arr.iter().enumerate() {
        let mut suma:i64 =0;
        for (x,operation_elemente)in arr.iter().enumerate(){
            if x==i {continue};
            suma+=*operation_elemente as i64
        }
        if suma>maximun{
            maximun = suma;
        }
        if suma<minimum{
            minimum = suma;
        }
    }
    println!("{} {}",minimum,maximun);
}