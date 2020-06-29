fn main() {
    //mean
    let numbers = vec![1,2,3,4,5,6,7];
    let sum : i32 = numbers.iter().sum();
    let mean : f32 = sum as f32 / numbers.len() as f32;    
    println!("{}",mean);

    //median
    let index = numbers.len() / 2;
    let mut median = 0;
    match numbers.get(index) {
        Some(value) => median = *value,
        _ => (),
    }
    println!("{}",median);

    use std::collections::HashMap;
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
}
