use std::collections::HashMap;

fn main() {
    let v = vec![5, 10, 15, 25];
    println!("Median for numbers: {:?} is equalt to {}", v, calculate_mean(&v));

    let med = vec![5, 7, 13, 14, 19, 26, 42, 5];
    println!("The median is {}", calculate_median(&med));
    println!("The value that occurs most often in the set is: {}", calculate_mode_value(&med));
}

// fn common_collection_vector() {
//
//     // DEFINING VECTOR
//     // remember when create a new variable that you don't assign a value to, you need to annotate
//     // the type it's gonna have!
//     // let v: Vec<i32> = Vec::new();
//
//     // if you insert values straight away Rust can infer the type
//     // Rust provides macro -> vac! for convenience. 
//     // vac! macro crate a new vector that holds the values you five it
//     // becuse we put i32 values, Rust can infer the type of v is i32!
//     let mut v = vec![1, 2, 3];
//
//     // UPDATING A VECTOR
//     // to update the vector must be mutable
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//
//     // as any other struct, a vector is freed when it goes out of scope
//     fn scope_for_new_v() {
//         let new_v = vec![1, 2, 3];
//         // do stuff with new_v in here
//     } // new_v goes out of scope in here
//
//     // READING ELEMENTS OF VECTOR
//     
//     // using & and [] gives us a reference to a value in the vector
//     let third: &i32 = &v[2];
//     // v.get method with the index as an arg gives us an Option<&T>
//     let third: Option<&i32> = v.get(2);
//     // the difference between two is that the first method will panic if given index out of bounds
//     // where the second with Option, will handle the case gracefuly returning None
//
//     // Borrow checkers and ownership
//     // Below won't compile, remember the you can't have mutable and immutable reference in the same
//     // scope!
//     let mut v_owner = vec![5, 6, 9];
//
//     let first = &v_owner[0];
//
//     // Below wont work!
//     // v_owner. push(6);
//
//     // ITERATE OVER THE VECTOR
//
//     // derefence operator * | to change the value that the mutable reference refers to, we have to
//     // use the dereference opeartor (*) to get the value in i before we can use += operator on it.
//     for i in &mut v_owner {
//         *i += 50;
//     }
//
//     // USE ENUM TO HOLD MULTIPLE DATA TYPES IN THE VECTOR
//
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }
//
//     // this is how we overcame the Vector limition to store a single type. 
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
//     
// }
// fn common_collection_string() {
//     
// }

fn calculate_mean(list: &Vec<i32>) -> i32 {

    let mut total = 0;
    let mut count = 0;

    for i in list {
        total += i;
        count += 1;
    }

    total/count

}

fn calculate_median(list : &Vec<i32>) -> u64 {

    let length = list.len();

    let median = (length + 1) / 2;
    median.try_into().unwrap()
}

fn calculate_mode_value(list: &Vec<i32>) -> i32 {

    let mut values = HashMap::new();

    for i in list {
        let count = values.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut times_it_occurs: i32 = 0; 
    let mut the_most_repeated_value: Option<i32> = None;
    for (&number, &value) in &values {
        if value > times_it_occurs {
            times_it_occurs = value;
            the_most_repeated_value = Some(number);
        } else {
            continue;
        }
    };

    let the_value: i32 = match the_most_repeated_value {
        Some(value) => value,
        None => 0
    };

    the_value
}
