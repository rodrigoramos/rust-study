use std::{
    fs::File,
    io,
    io::{ErrorKind, Read},
};

fn main() {
    let username = read_username_from_file_with_shortcut()
        .expect("Não consegui ler o nome do usuário pelo arquivo");

    println!("Nome do usuário é {:?}", username);
}

fn read_username_from_file_with_shortcut() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _with_expect() {
    let _f = File::open("hello.txt").expect("Não consegui abrir o arquivo");
}

fn _with_unwrap() {
    let _f = File::open("hello.txt").unwrap();
}

fn _with_unwrap_or_else() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _with_math() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
