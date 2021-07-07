// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec = Vec::new();

    fill_vec(&mut vec);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
    vec.push(88);
    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
}

fn fill_vec(vec: &mut Vec<i32>){

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
