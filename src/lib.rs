use reqwest::header::HeaderMap;

#[cfg(test)]
mod tests {
    
    fn setup_client() -> reqwest::Client {
        let mut headers = HeaderMap::new();
        headers.insert("ACCEPT", "application/json".parse().unwrap());
        headers.insert("ContentType", "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        return client;
    }

    #[test]
    fn test_shorten() {
        let client = setup_client();
        let src: String = "int main() { return 0; }".to_string();
        let result = shorten(client, "https://godbolt.org", src, "g91", "-O1".to_string());
        assert_eq!(result, "https://godbolt.org/z/CJ1Nvy".to_string());
    }

}