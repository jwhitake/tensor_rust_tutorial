
// #![allow(dead_code)]

// #[derive(Debug)]
// enum Direction{
//     Up(Point),
//     Down(Point),
//     Left(Point),
//     Right(Point)
// }

// #[derive(Debug)]
// enum Keys{
//     UpKey(String),
//     DownKey(String),
//     LeftKey(String),
//     RightKey(String)
// }

// impl Direction{
//     fn match_direction(&self) -> Keys{
//         match *self{
//             Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
//             Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
//             Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
//             Direction::Right(_) => Keys::UpKey(String::from("Pressed d"))
//         }
//     }
// }

// impl Keys{
//     fn destruct(&self) -> &String{
//         match *self{
//             Keys::UpKey(ref s) => s,
//             Keys::DownKey(ref s) => s,
//             Keys::LeftKey(ref s) => s,
//             Keys::RightKey(ref s) => s,
//         }
//     }
// }

// #[derive(Debug)]
// struct Point{
//     x: u32,
//     y: u32
// }
// fn main() {
//     let u = Direction::Up(Point{x: 0, y: 1});
//     let k = u.match_direction();
//     let x = k.destruct();

//     println!("{:?}", k);
//     println!("{:?}", x);
// }

// enum Option<T>{
//     Some(T),
//     None
// }


fn division(x: f64, y: f64) -> Option<f64>{
    if y == 0.0 {
        None
    }else{
        Some(x/y)
    }
}

fn main(){
    let res = division(5.0, 7.0);
    match res{
        Some(x) => println!("{:.10}", x),
        None => println!("Cannot device by 0")
    }
}