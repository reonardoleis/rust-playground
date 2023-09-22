fn main() {
    let mut s1 = String::from("Hello, ");
    let mut s2 = foo( &s1);


    println!("{} {}", s1, s2);

    s1 = String::from("world!");
    s2 = foo(&s1);

    println!("{} {}", s1, s2);
}

fn foo(str: &String) -> &str {
    return &str[..];
}