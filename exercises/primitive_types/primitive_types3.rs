// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM NOT DONE

fn main() {
    let a: [i32; 100] = [0; 100];

    for x in a {
        println!("{}", x + 1)
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array! {:?}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }

}
