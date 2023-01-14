fn print_str(arg: &str){
    println!("arg {}", arg)
}
fn main() {
    print_str("Hello world");
    print_str(&String::from("Hello "))

}