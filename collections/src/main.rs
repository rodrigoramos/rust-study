fn main() {
    hashmap();
}

fn hashmap() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // println!("Values {}", scores["Blue"]);
    // for (key, value) in scores {
    // println!("{}: {}", key, value);
    // }

    let a = scores.iter().next();
    match a {
        Some((key, value) -> println!("{}: {}", key, value),
        None => println!("Fim"),
    }
}

fn strings_iteration() {
    let str1 = String::from("d'águação");

    let si = 2;
    let fi = 7;

    let str2 = &str1[si..str1.len()];

    // for s in str1.chars() {
    //     println!("{}", s);
    // }

    println!("{}", str2);
}

fn strings() {
    let s1 = String::from("Rodrigo");
    let s2 = String::from("Ramos");

    let result = s1 + &s2;

    println!("Result {}", result);
    println!("V2 {}", s2);
    // println!("Variáveis {} {}", s1, s2);
}

fn collections() {
    println!("Collections");

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 100;
    }

    for i in &v {
        println!("Item {}", i);
    }
}
