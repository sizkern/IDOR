use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    for i in 0..20 {
        let url = format!("http://testphp.vulnweb.com/artists.php?artist={}", i);
        match client.get(&url).send() {
            Ok(response) => {
                if let Ok(text) = response.text() {
                    if text.contains("lyzae") {
                        println!("Found: {}", url);
                    }
                }
            },
            Err(e) => println!("Error {}: {}", i, e),
        }
    }
}
