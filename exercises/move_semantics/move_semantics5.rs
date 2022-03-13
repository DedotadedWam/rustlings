// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// you cannot borrow(&) a variable two times and dereference(*) the first.


fn main() {
    let mut x = 100;
    let z = &mut x;
    *z += 1000;
    let aa = &mut x;
    *aa += 50;
    assert_eq!(x, 1150);
}
