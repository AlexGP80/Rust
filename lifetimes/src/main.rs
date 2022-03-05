#![allow(unused)]
fn main() {
<<<<<<< HEAD
    let x = 5;
    let r = &x;

    println!("r: {}", r);
}
=======
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
>>>>>>> 8b549a2dff04a903f7e888654a8e7171d044a6e4
