mod vec2;
use vec2::build_vec2;


fn main() {
    let myvec1 = build_vec2(1.0, 1.0);
    let myvec2 = build_vec2(128.0, 64.0);

    println!("myvec1 vec2 [{},{}]", &myvec1.x, &myvec1.y);
    println!("distance is {}", &myvec1.distance_to(&myvec2));

    println!("sum of two vectors is {}", &myvec1 + &myvec2);
    println!("dif of two vectors is {}", &myvec2 - &myvec1);
    println!("mul of two vectors is {}", &myvec1 * &myvec2);

    println!("length of myvec1 is {}", myvec1.length());
    println!("norm myvec1 vector is {}", myvec1.normalized());
}
