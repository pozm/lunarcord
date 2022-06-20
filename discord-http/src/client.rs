struct DiscordHttpClient {
    client: reqwest::Client,
    token: String,
    user:bool,
    api_ver:i8
}
impl DiscordHttpClient {
    pub const BASE_URL: &'static str = "https://discord.com/api";
    pub fn new(token: String,user:bool) -> DiscordHttpClient {
        DiscordHttpClient {
            client: reqwest::Client::new(),
            token,
            user,
            api_ver:9
        }
    }
    pub async fn make_request(&self,url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("{} {}", if self.user {"bearer"} else {"bot"}, self.token)).unwrap(),
        );
        self.client.get(format!("{}/{}",DiscordHttpClient::BASE_URL,url)).headers(headers).send().await
    }
    pub async fn get_user(&self) {
        self.make_request("users/@me").await
    }
}