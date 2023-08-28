use ureq::{json, Agent};

fn create_post_on_telegraph(
    access_token: &str,
    title: &str,
    content: &str,
) -> Result<(), ureq::Error> {
    let agent = Agent::new();

    let response = agent
        .post("https://api.telegra.ph/createPage")
        .timeout_read(5_000)
        .query("access_token", access_token)
        .send_json(json!({
            "title": title,
            "content": content,
            "author_name": "Your Name",
        }))?;

    if response.status() == 200 {
        let body = response.into_string()?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        // Extract and use the necessary fields from the JSON response if needed
        // For example, you can get the URL of the created page:
        // let url = &json_data["result"]["url"];
        // println!("Post created successfully. URL: {}", url);

        Ok(())
    } else {
        Err(ureq::Error::Status(
            response.status(),
            Some(response.into_string()?),
        ))
    }
}

fn main() {
    let access_token = "YOUR_TELEGRAPH_ACCESS_TOKEN";
    let title = "My Telegraph Post";
    let content = "<p>Hello, Telegraph!</p>";

    match create_post_on_telegraph(access_token, title, content) {
        Ok(()) => println!("Post created successfully."),
        Err(err) => eprintln!("Failed to create post: {}", err),
    }
}
