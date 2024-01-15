fn main() {
    let x = 5;
    {
        let x = 7;
        println!("{}",x);
    }
    println!("{}",x);
   
}
