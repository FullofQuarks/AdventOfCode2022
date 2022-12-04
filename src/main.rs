pub mod utilities;
pub mod days;

fn main() {
    println!("Input a day: ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("msg");
    println!("Executing Day {} of Santa Protocol...", &choice.trim());
    match &choice.trim()[..] {
        "1" => crate::days::day_01::execute(),
        "2" => crate::days::day_02::execute(),
        _ => println!("None chosen. Exiting.")
    }
}
