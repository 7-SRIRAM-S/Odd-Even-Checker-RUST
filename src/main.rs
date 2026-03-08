use std::io;

fn main() {
    let mut num=String::new();
    println!("Enter any number : ");
    io::stdin().read_line(&mut num).expect("Enter valid number");
    let num:i32 = num.trim().parse().expect("not a number");
    if (num&1)==0 {
        println!("The number {} is even",num);
    }
    else {
        println!("The number {} is odd",num);
    }
}
