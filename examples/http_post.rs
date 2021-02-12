use reqwest;

fn main() {
    let client = reqwest::blocking::Client::new();

    let token = "123";

    let post = client
        .post("https://www.baidu.com")
        .header("Authorization", format!("Bearer {}", token));

    let req = post.send().unwrap();
    println!("{:?}", req);
    let text = req.text().unwrap();
    println!("html: {}", text);
}
