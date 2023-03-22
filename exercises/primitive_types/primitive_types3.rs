// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a= [1, 2, 3];
 //    for x in a.iter() {
 //        println!("{}", x);
 //    }
 //    
 //    let b = vec![1,2,3];
 //    for x in b {
 //        println!("{}", x);
 //    }
 // 

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
