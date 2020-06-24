
// the pub keyword make it visible outside the file
// where it is delcared
pub fn hello() {
    let x  = 0x5;
    println!("hello");
    println!("{}",x);
    //boolean
    let y: bool = true;
    println!("{}",y);
    // unsigned 32
    let y = 43u32;
    println!("{}",y);
    //float
    let y = 43.3f32;
    println!("{}",y);
    //uint8
    let y = 5u8;
    println!("{}",y);
    //binary
    let y = 0b1010;
    println!("{}",y);
    //binary
    let y = 0b1010_0101;
    println!("{}",y);
}

// https://github.com/nrc/r4cppp/blob/master/primitives.md