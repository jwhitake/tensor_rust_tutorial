//use std::collections::HashMap;
use std::fs::File;

// #[derive(Debug)]
// enum Example {
//     Float(f64),
//     Int(i32),
//     Text(String)
// }


fn main() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error)
        }
    };

    

    // let mut s = Some(0);
    // // loop{
    // //     match s {
    // //         Some(i) => if i > 19{
    // //             println!("Quit";
    // //             s = None;)
    // //         } else{
    // //             println!("{}", i);
    // //             s = Some(i + 2);
    // //         },
    // //         _ => {
    // //             break;
    // //         }
    // //     }
    // //}

    // while let Some(i) = s {
    //     if i > 19 {
    //         println!("Quit");
    //         s = None;
    //     }else{
    //          println!("{}", i);
    //          s = Some(i + 2);
    //     }
    // }


    // let mut hm = HashMap::new();

    // hm.insert(String::from("random"), 12);
    // hm.insert(String::from("strings"), 49);

    // for (k, v) in &hm{
    //     println!("{}: {}", k, v);
    // }


    // match hm.get(&String::from("doofus")){
    //     Some(&n) => println!("{}", n),
    //     None => println!("No match")
    //     //_ => println!("no match")
    // }


    // let r = vec![
    //     Example::Int(142),
    //     Example::Float(12.32),
    //     Example::Text(String::from("string"))
    // ];

    // println!("{:#?}", &r);

    // // let x= vec![1, 2, 3, 4];
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // for i in &v {
    //     println!("{}", i);
    // }

    // println!("{:?} {} {}", &v, v.len(), v.capacity());
}
