use evalexpr::*;

fn main() {
    println!("hello");
    let s1 = " old * old ";
    let s2 = " old *  2";
    let s3 = " old  + 10";

    let t = 3;

    let a1 = eval(&s1.replace("old", &t.to_string())).unwrap();
    let a2 = eval(&s2.replace("old", &t.to_string())).unwrap();
    let a3 = eval(&s3.replace("old", &t.to_string())).unwrap();

    println!("{}, {}, {}", a1, a2, a3);
    let a4 = a1.as_int().unwrap() + 3;
    println!("{a4}");
}
