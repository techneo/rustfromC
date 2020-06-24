//https://play.rust-lang.org/?version=stable&mode=release&edition=2018
//https://github.com/nrc/r4cppp/blob/master/control-flow.md
//https://doc.rust-lang.org/1.27.2/book/first-edition/guessing-game.html

// -> means the return value

// like #include 
mod test;

fn main() -> () {

    // mut is like a variable
    let mut name: &'static str  = "suren";

    let x : Vec<i32> = [ 1 , 2, 3, 4].to_vec();

    let mut y : Vec<i32> = [ 1 , 2, 3, 4].to_vec();

    // println rust equivalent for printf
    // The ! means that it is a macro
    // the {} is the delemeter like  %d %s etc in C
    println!("Hello, {}!",name);
    // this is different from C where string has to be copied to buffer
    // but in CPP it might be posible.
    name = "x";
    // similar to printf("Hello %s !",test(name));
    println!("Hello, {}!",test(name));
    println!("number is {}",check(15));
    count(5);
    list(x);
    double_all(&mut y);
    list(y);
    println!("1 is {}",print_some(1));
    print_some1(12);
    print_some_more(1);
    print_some_more(7);
    print_some_more(10);
    print_some_more(12);
    // test is included using the mod keyword.
    test::hello();
}

// -> returns a string
// when a function body does not have anything but a 
// single statement it is considered as the return statement
// [todo] need to find what _x means
// _ in a variable declaration indictes unnamed variable
fn test(_x: &'static str) -> &'static str {
    return "enjoy";
}

fn check(x:i32) -> &'static str {
    if x > 10 {
        //function returning a string need not use "return"
        "greater than 10"
    }
    else {
        "less than 10"
    }
    
}

// returns void

fn count(mut x:i32) -> () {
    // con ditional control statements
    while x >0 {
        println!("counting {}",x);
        x -= 1;
    }
}

fn list(all: Vec<i32>) -> () {
    
    // this is a iterator
    // used mostly for complex data types
    for a in all.iter() {
        println!("{}",a);
    }

    // this is also a iterator like c For loop
    // for i = 0 to n type of implementation
    for i in 0..all.len() {
        println!("{}:{}",i,all[i]);
    }

    // we dont need a separate counter for i 
    // rather we can get it from then the enumerate() chain method 
    // of iter().

    for (i,a) in all.iter().enumerate() {
        println!("{},{}",i,a);
    }
    
}

// this is suposed to expalin a method called
// "Borrowed Pointers" that will be explained
// later
fn double_all(all: &mut Vec<i32> ) {
    for a in all.iter_mut() {
        *a += *a;
    }
}

// returns string
fn print_some(x:i32) -> &'static str {
    // match has some similarities to case 
    match x {
        0 => "zero",
        1 => "one",
        // _y implies it us not used anywhere
        // this case is needed for compiling match without warning
        // match has to cover the entire range of the variable -i32max to +i32max
        // _ => {} is also valid 
        _y => "something else"
    // this ; plays a crutial role when this function 
    // has to retun something.

    }
}
    

// no return 
fn print_some1(x:i32)  {
    // match has some similarities to case 
    match x {
        0 => println!("zero"),
        1 => println!("one"),
        // _y implies it us not used anywhere
        // this case is needed for compiling match without warning
        // match has to cover the entire range of the variable -i32max to +i32max
        y => println!( "{} is something else",y)
    // ; is not needed as the return type is void
    };
}

fn print_some_more(x:i32)
{
    let message = match x {
        0 | 1 | 10 => "one or zeor or 10",
        y if y < 10 => "not zero or one or ten and less than ten",
        y if y == 100 => "hundred",
        _ => "something else"
    // note the semicolon
    };

    println!("{} is {}",x,message);
}