use reqwest::{header::HeaderValue, ClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build()?;
    let response = client
        .head("http://tehetseg.inf.elte.hu/nemes-online/nemes-aktualis.html")
        .send()
        .await?;
    let last_modified = response.headers().get("Last-Modified");

    let unchanged = &HeaderValue::from_static("Mon, 13 Feb 2023 07:40:56 GMT");
    let message = match last_modified {
        Some(last_modified) => String::from(if last_modified == unchanged {
            "unchanged"
        } else {
            "oh?"
        }),
        None => response.status().to_string()
    };

    println!("{}", message);

    Ok(())
}
