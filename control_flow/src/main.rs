fn main() {

    let x = 25;

    match x {
        1 => println!("{}", 1),
        2 => println!("{}", 2),
        3 => println!("{}", 3),
        4 => println!("{}", 4),
        5 => println!("{}", 5),
        11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 => println!("This is a teen"),
        20 ... 29 => println!("This is in the 20s.  number: {}", x),
        _ => println!("Default")
    }


    // for i in 1..101{
    //     println!("i: {}", i);
    // }


    // let c = 7;

    // let n = if c < 9{
    //     1
    // }else{
    //     0
    // };
    // println!("{}", n);
}
