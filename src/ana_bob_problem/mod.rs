use std::cmp::Ordering;

pub fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut arr_response = vec![0,0];
    for i in  0..a.len(){
        match a[i].cmp(&b[i]){
            Ordering::Less=>arr_response[1]+=1,
            Ordering::Greater=>arr_response[0]+=1,
            Ordering::Equal=>(),
        };
    }
    arr_response
}
// let result = compareTriplets(&[1,2,3],&[4,2,1]);
