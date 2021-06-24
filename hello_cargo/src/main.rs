fn main() {
    println!("Hello, world!");

    let arr = [1, 2];

    another_stuff(&arr);
}

fn another_stuff (arr: &[i32]) {
    let index = 2;
    println!("Exi {}", arr[index]);
}
