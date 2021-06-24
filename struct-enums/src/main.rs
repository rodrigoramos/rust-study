// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call (&self) {
//     }
// }

// impl Rectangle {
//     // fn area(&self) -> u32 {
//     //     self.width * self.height
//     // }

//     fn can_hold(&self, candidate: &Rectangle) -> bool {
//         (self.width >= candidate.width && self.height >= candidate.height) ||
//         (self.width >= candidate.height && self.height >= candidate.width)
//     }

//     fn square (size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size
//         }
//     }
// }

fn main() {

    // let msg = Message::Write(String::from("meu texto"));
    // msg.call();
    //

    let valor = Some(1);

    // match valor {
    //     Some(0) => println!("zero"),
    //     Some(1) => println!("um"),
    //     Some(i) => println!("numero {}", i),
    //     None => ()
    // }

    match valor.unwrap_or(0) {
        0 => println!("zero"),
        1 => println!("um"),
        5..=10 => println!("de 5 até 10"),
        i => println!("numero {}", i),
    }

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };
    // let rect4 = Rectangle::square(12);

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}
