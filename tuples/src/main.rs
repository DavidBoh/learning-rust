fn main() {
    /*  explicit type annotation
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    */

    /*
    let tup = (500, 6.4, 1);
    let (_x,y, _z) = tup;

    println!("The value of y is: {}",y);
    */

    let x : (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    println!("The value of x.0 is {} x.1 is {} and x.2 is {}",
            five_hundred,six_point_four,one);
}
