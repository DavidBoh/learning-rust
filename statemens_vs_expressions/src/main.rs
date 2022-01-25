fn main() {
    let x = 5;

    {
        let aplha: u8 = 20;
        println!("alpha = {}",aplha);
    }//scope


    let y = {
        let x = 3;
        println!("The value of x is {} ", x);
        x + 1
    };
    println!("The value of x is {} and y is {}",x,y);
}
