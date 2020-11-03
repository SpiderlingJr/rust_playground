//mod some_basics;
//mod guessing_game;

mod some_algos;
//mod some_basics;
//use some_basics::ownership;
//use some_algos::fib;

fn main() {
    println!("Hello, world!");
    //some_basics::basics()
    //guessing_game::guessing_game()
    //some_algos::fib::fibo(3);

    //some_basics::ownership::dangling_references();

    //let mut my_arr = [4,3,1,5,2];
    let mut my_arr = [4,7,8,3,10,9,1,5,2,6];
    println!("Pre Sort {:?}",my_arr);
    some_algos::sort_algorithms::bubble_sort_1(&mut my_arr);
    println!("Post Sort {:?}",my_arr);
}
