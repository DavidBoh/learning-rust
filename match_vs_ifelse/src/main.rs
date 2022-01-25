fn main() {
    let number = 6;

    match number {
       ((number % 4) == 0) => println!("Number is divable by 4");
       ((number % 3) == 0) => println!("Number is divable by 3");
       ((number % 2) == 0) => println!("Number is divable by 2");
    }
}
