// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)



/* 

Read https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
But mutable references have one big restriction: 
you can have only one mutable reference to a particular piece of data at a time. 
The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition 
and happens when these three behaviors occur:

*/

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    print!("x {}",x);
    let z = &mut x;
    print!("z {}",z);
    *z += 1000;
    assert_eq!(x, 1200);
}
