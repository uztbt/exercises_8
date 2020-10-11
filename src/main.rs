mod list_of_ints;
mod pig_latin;
mod directory;

fn main() {
    // List of ints

    let v = vec![1,4,-3,5, 1];
    
    let mean = list_of_ints::mean(&v);
    let median = list_of_ints::median(&v);
    let mode = list_of_ints::mode(&v);
    
    println!("mean of {:?} is: {}", &v, mean);
    println!("median of {:?} is: {}", &v, median);
    println!("mode of {:?} is: {}", &v, mode);

    // Pig Latin

    let mut first = "first".to_string();
    if let Some(pig_first) = pig_latin::convert(&mut first) {
        println!("The pig latin of first is {}", pig_first);
    }

    let mut apple = "apple".to_string();
    if let Some(pig_apple) = pig_latin::convert(&mut apple) {
        println!("The pig latin of apple is {}", pig_apple);
    }
}