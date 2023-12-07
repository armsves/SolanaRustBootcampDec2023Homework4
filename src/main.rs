fn main() {
    println!("Welcome to fizz buzz rust program");

    fn fizzbuzz() {
        let mut x:u32 = 0;
        for n in 1..301 {
            if n % 3 == 0 { println!("fizz"); }
            if n % 5 == 0 { println!("buzz"); }
            if n % 15 == 0 { 
                println!("fizzbuzz"); 
                x += 1;
            }
        }
        println!("fizzbuzz was called {} times", x)
    }
    fizzbuzz()
}
