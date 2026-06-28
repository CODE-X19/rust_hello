/*fn main() {
    println!("hello world");
let rect = area((23,23));
print!(
    "the area of the rectangle is {} square meter",rect
)
}
fn area(dimention: (u32,u32))->u32{
    dimention.0 *dimention.1
}*/
// CONTROLFLOW

//IF ELSE IF ELES STATEMENT

/*fn main() {
    let varrs = true;
    let _number = if varrs { 5 } else { 6 };
    println!("{}", _number);


    let temp = weather(25);
    println!("weather:{}", temp);

fn weather(tem: i32) -> &'static str {
    if tem < 20 {
        "cold"
    } else if tem <= 30 {
        "warm"
    } else {
        "hot"
    }
}
}*/

// LOOP

/*fn main() {
    loop {
        println!("i write rust");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("counter is : {}", result);

    let mut count = 0;
    loop {
        count += 1;
        println!("count is {}", count);
        if count == 10 {
            break;
        }
    }
}*/

// fn main() {
// let mut number = 0;
// loop {
//     number += 1;
//     println!("{}", number);
// }

// let mut number = 0;
// loop {
//     number += 1;
//     println!("{}", number);

//     if number == 10 {
//         break;
//     }
// }
// }

//WHILE LOOP

/*fn main() {
    let mut number = 10;

    while number != 0 {
        println!("number is {}", number);
        number -= 1;
    }
    println!("liftoff");

    let mut number = 0;
    while number < 10 {
        println!("number is {}", number);
        number += 1
    }
    println!("liftoff");
}
*/
// fn main() {
// let mut number = 5;
// while number != 0 {
//     println!("{}", number);
//     number -= 1;
// }
// println!("done")
//     let mut number = 0;
//     while number != 10 {
//         println!("{}", number);
//         number += 2;
//     }
//     println!("done")
// }

//FOR LOOP

/*fn main() {
    for number in 1..=10 {
        println!("{}", number);
    }

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for item in a.iter() {
        println!("the number is {}", item);
    }
     let fruits = ["Apple", "Mango", "Orange"];
    for items in fruits.iter() {
        println!("{}", items)
    }
}*/

//STRUCT
/*#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
fn main() {
    let user1 = build_user(
        String::from("victorolatunde70@gmail.com"),
        String::from("victor001"),
    );
    let user2 = build_user(
        String::from("Michealgate@gmail.com"),
        String::from("michealgate"),
    );
    println!("user 2:{:?}", user2);
    let user3 = User {
        email: String::from("jemimahbenard.com"),
        username: String::from("jemimah"),
        ..user2
    };
    println!("user1 :{:?}", user1);

    println!("user 2: {:?}", user3);
}*/
// fn main() {
//     let mut user1 = User {
//         email: String::from("victorolatunde70@gmail.com"),
//         username: String::from("Victor0001"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let name = user1.username;
//     println!("{}", name);
//     user1.username = String::from("jemimah002");
//     println!("{}", user1.username);

//     let user2 = build_user(
//         String::from("benardjemimah@gmail.com"),
//         String::from("oluwatosin001"),
//     );

//     let user3 = User {
//         email: String::from("michaelgate@gmail.com"),
//         username: String::from("Michelgate001"),
//         ..user2

//     };

// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         username,
//         email,
//         active: true,
//         sign_in_count: 1,
//     }
// }

/*struct Rectangle {
    width: u32,
    hight: u32,
}
fn main() {
    let rect = Rectangle {
        width: 25,
        hight: 23,
    };

    impl Rectangle {
        fn area (&self)->u32{
            self.width*self.hight
        }

    }

    println!("the area is {} squar meter", rect.area());
}*/

//ENUMS

/*enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn some_function() {
        println!("am a rustasian")
    }
}

struct IpAdd {
    kind: IpAddKind,
    address: String,
}
fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    let localHost = IpAddKind::V4(127, 0, 0, 1);
}


//OPTION ENUM

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin:Coin)->u8{
    match coin {
        Coin::Penny=> {
            println!("Lucky Pennt");
            1
        }
        Coin::Nickel=> 5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("tste quater from{:?}",state);
            25
        }

    }
}*/

//VECTORS

/*fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];

    println!("the third element is {}", third);

    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element"),
    }
}*/

//STRING

//in rust strings are stored as collection of UTF-8 encodes bytes

//ERROR HANDLING

/*use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn ");

    // enum Result<T, E> {
    //     OK(T),  //-> SUCCESS
    //     Err(E), //-> FAILURE
    // }

    let f = File::open("hello.txt"); //::open returns a result type because opening a file may fail and can be sussessful

    //useing shadowing to declear f

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file{:?}", e),
            },
            other_Error => {
                panic!("problem opening the file {:?}", other_Error)
            }
        },
    };
}*/

//MEMORY MANAGEMENT

// OWNERSHIP

//Rule 1: Each Value in rust as an owner
// let s1 = String::from("this is victor");// in this expression s1 owns the string

// rule 2 : there can only be one owner at a time
// let s1 = String::from("this is victor");
//let s2 = s1;
//println!("{}",s1) if we TRIE TO PRINT S1 we will get a compile time error because the ownership as been transferd to s2 so s1 is invalid
// because the new verabile (s2) is = s1,the ownership of the string as been transferd from s1 to s2
// Rule 3 : when the owner goes out of scop,  the value will be droped

//REFRENCE : this is denoted with (&)
//Creating refrences in rust is called borrowing
//& borrows data from the owner just to access it without owning it or without taken oownership of it
// here are BORROWING RULES
//Rule 1: at any given time,you can either have one mutable reference  or any number of immutable references.
//rule 2: Refreences must always be valid
// fn main() {
//     let s1 = String::from("i am victor ");
//     print_string(&s1);
//     print_string(&s1);
// }

// fn print_string(s: &str) {
//     println!("{s}",);
// }

//LIFETIME:this referes to the span of the program which a specific place of a data is valid

//for loop

// fn main() {
//     for n in 1..=100 {
//         if n % 15 == 0 {
//             println!("it is victor ");
//         } else if n % 3 == 0 {
//             println!("it is also victor");
//         } else if n % 5 == 0 {
//             println!("it is only victor");
//         } else {
//             println!("{}", n);
//         }
//     }
// }

//iterators : an iterator is simply what makes you go through items one by one in an arrays, vectors, slices, strings, hash maps, and many other collections in Rust.
// there are 3 types of iterator
//1: iter: this type of iterator only let you borrow and read  you can't modify or edit
// example in a code

/*fn main() {
    let number = vec![1, 2, 3];
    for num in number.iter() {
        println!("{}", num);
    }
// rust only give you a refrence of number it means num == &1, &2, &3
// you can only read it
//the vector still belong to number not num, num only borrow it
    //note you can only print it out you cant modify if you do anything like num *=2 you will get a compile time error
}*/

//2: iter_mut() : what iter_mut does is that it let you borrow the vector and also allow you modify it or change the value in the vector or in the collection.
//example
/*fn main() {
    let mut number = vec![1, 2, 3];
    for num in number.iter_mut() {
        *num *= 2;
        println!("{}", num);
    }

    // here rust gives you a refrence of the vector and let you modify it
    //you can read and modify
}*/

//3) is into_iter : whatg into_iter does is that it takes ownership of the vector or collection , it does not only iterate after iterating one after the other it take it away.
// example

/*fn main() {
    let number = vec![1, 2, 3];
    for num in number.into_iter() {
        println!("{}", num);
    }
    // here rust take the ownership from number and give it to num completly
    // also after for num in number.into_iter(), number becomes empty the vectors are gone
    // if you tey to do something like this :
    println!("{:?}", number);
    //you will get a compile time error because number is empty, the ownership as been transfered to "num"
}*/

//INPUTE / OUTPUT

use std::io;

/*fn main() {
   let mut name = String::new();
   println!("pleas enter your name ");
   io::stdin().read_line(&mut name).unwrap();
   println!("hello {}",name);
}*/

/*fn main() {
    println!("kindly input your age ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).unwrap();
    let age: i32 = age_input.trim().parse().unwrap();

    if age < 13 {
        println!("You are a child")
    } else if age >= 13 && age <= 19 {
        println!("You are a teenager")
    } else if age >= 20 && age <= 59 {
        println!("You are an adult")
    } else if age >= 60 {
        println!("You are a senior citizen")
    }
}
*/

/*fn main (){
    println!("enter your username below");
    let mut username_input = String::new();

    let mut password_input = String::new();
    io::stdin().read_line(&mut username_input).unwrap();
    println!("enter your password below");
    io::stdin().read_line(&mut password_input).unwrap();
    let username = "Admin";
    let password = "rust123";

    if username_input == username && password_input == password {
        println!("Access granted")
    }else {
        println!("Access denied")
    }
}*/

/*fn main() {
    let mut bill_input = String::new();
    let mut senior_citizen_input = String::new();
    println!("input your bill");
    io::stdin().read_line(&mut bill_input).unwrap();
    println!("input yes or no if you are a senior citizen");
    io::stdin().read_line(&mut senior_citizen_input).unwrap();
    let bill: i32 = bill_input.trim().parse().unwrap();
    if bill > 10000 {
        if senior_citizen_input.trim() == "yes" {
            let discount = bill * 20 / 100;
            println!("your diacount is: {}", discount);
        }
    } else {
        println!("No discount")
    }
}*/

/*fn main() {
    println!("input your word");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let plindrome: String = input.to_lowercase().trim().chars().rev().collect();
    if input.to_lowercase().trim() == plindrome {
        println!(
            "your imput is plindrome because {} in reverse is {} ",
            input, plindrome
        );
    } else {
        println!("your imput is not plindrome ")
    }
}
*/

fn main() {
    let mut name_input = String::new();

    while name_input.trim() != "bye" {
         name_input.clear();
        io::stdin().read_line(&mut name_input).unwrap();

        println!("{}", name_input.trim());
    }
    println!("come again soon")
}
