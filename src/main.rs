pub mod hash;

extern crate hex;


fn main() {
    let mut line = String::new();
   
    println!("Enter a string :");
   
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();

    println!("{}", hash::hash(line.clone()));
}
