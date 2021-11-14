use actix_web::client::Client as ActixClient;

pub struct Client {
    actix_client: ActixClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            actix_client: ActixClient::default(),
        }
    }

    pub async fn get(&self, path: &str) -> String {
        // NOTE: Rustで日本語の入っているURLはアクセスできないのでpathは英語であること。あとでちゃんと調べる
        let url = format!("https://www.google.com/search?q={}+kashi", path);
        let mut response = self.actix_client.get(url).send().await.unwrap();
        let byte = response.body().await.unwrap();
        let body = byte.iter().map(|&s| s as char).collect::<String>();

        body
    }
}