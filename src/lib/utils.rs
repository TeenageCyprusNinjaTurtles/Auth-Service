use std::io::Read;

use rouille::Request;

pub fn request_to_bytes(request: &Request) -> Vec<u8> {
    let mut buffer = Vec::new();
    request.data().unwrap().read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn get_user_level(request: &Request) -> i32 {
    let mut headers = request.headers();
    let level = headers.find(|&k| k.0 == "X-User-Level");
    if level.is_none() {
        return 0;
    }
    let level: i32 = level.unwrap().1.parse().unwrap_or(0);
    level
}

pub fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let token: String = (0..32)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    token
}