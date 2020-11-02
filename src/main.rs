//mod some_basics;
//mod guessing_game;

mod some_algos;
mod some_basics;

use some_basics::ownership;
use some_algos::fib;

fn main() {
    println!("Hello, world!");
    //some_basics::basics()
    //guessing_game::guessing_game()
    //some_algos::fib::fibo(3);

    some_basics::ownership::reference_and_borrowing();
}
