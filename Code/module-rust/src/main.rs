mod sett;
fn main() {
    println!("{}", sett::compile(sett::Request(String::from("https://raw.githubusercontent.com/Kolya142/my-prog-settings/main/test.sett"))));
}