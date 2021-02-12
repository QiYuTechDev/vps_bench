use reqwest;

fn main() {
    let req = reqwest::blocking::get("https://www.baidu.com").unwrap();
    println!("{:?}", req);
}
