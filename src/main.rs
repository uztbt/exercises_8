mod list_of_ints;
mod pig_latin;
mod directory;

fn main() {
    let v = vec![1,4,-3,5, 1];
    
    let mean = list_of_ints::mean(&v);
    let median = list_of_ints::median(&v);
    let mode = list_of_ints::mode(&v);
    
    println!("mean of {:?} is: {}", &v, mean);
    println!("median of {:?} is: {}", &v, median);
    println!("mode of {:?} is: {}", &v, mode);
}