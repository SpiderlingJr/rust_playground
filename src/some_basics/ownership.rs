pub fn ownership() {


    // Immutable String, for hardcoded literals, stored on stack
    let a = "hello";
    // Mutable String, for user input, stored on heap
    let mut b = String::from("helloooo");


    // Moving variables

    let s1 = String::from("Holdmybeer");
    let s2 = s1;

    // This will fail, because the value of s1 has been moved to s2!
    //println!("{} from s1", s1);

    // Copying variables

    let s1 = String::from("Thisishalloween");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("Hello");
    let x = 5;

    takes_ownership(s);
    // s is out of scope now!
    // println!("{}", s);

    makes_copy(x);
    // x still in scope


    // Return Values and Scope
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    } // Calls drop of some_string, thus freeing memory

    fn makes_copy(some_int: i32) {
        println!("{}", some_int);
    }

    fn gives_ownership() -> String {
        let some_string = String::from("Hello");
        return some_string;
    }

    fn takes_and_gives_back(a_string: String) -> String {
        return a_string;
    }
}   // s1 goes out of scope
// s2 was moved, nothing happens
// s3 goes out of scope

pub fn reference_and_borrowing() {
    // When wanting to retain a value that has been earlier passed into a function,
    // instead of manually passing it back via returning tuples or so,
    // one can use references.

    let s1 = String::from("hello");
    let len = get_len(&s1);

    println!("Length of '{}' is {}.", s1, len);

    fn get_len(s: &String) -> usize { // s is a refernce to a String
        return s.len()
    } // s goes out of scope.
    //But it doesnt have ownership of what it refers to, so nothing happens.
}