fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
