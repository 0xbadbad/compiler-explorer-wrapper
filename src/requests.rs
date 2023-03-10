use reqwest::Client;

use crate::compiler::Compiler;
use crate::language::Language;
use crate::source::Output;

const API_URL: &str = "https://godbolt.org";

pub fn get_languages(client: Client, host: &str) -> Vec<Language> {
    client
        .get(format!("{}/api/languages", host).as_str())
        .send()
        .expect("Failed to commit transaction in get_languages")
        .json()
        .expect("Failed to parse JSON in get_languages")
}

pub fn get_compilers(client: &Client, host: &str, language: Option<&str>) -> Vec<Compiler> {
    match language {
        Some(lang) => client
            .get(format!("{}/api/compilers/{}", host, lang).as_str())
            .send()
            .unwrap()
            .json()
            .unwrap(),
        None => client
            .get(format!("{}/api/compilers/", host).as_str())
            .send()
            .unwrap()
            .json()
            .unwrap(),
    }
}

pub fn compile(client: Client, host: &str, src: String, compiler: &str, args: String) -> String {
    let filters = json!(
        {
            "intel": true,
            "demangle": true,
            "directives": true,
            "comments": true,
            "labels": true
        }
    );

    let options = json!({
        "userArguments": args,
        "filters": filters
    });

    let source = json!({
        "source": src,
        "options": options
    });

    let output: Output = client
        .post(format!("{}/api/compiler/{}/compile", host, &compiler).as_str())
        .json(&source)
        .send()
        .unwrap()
        .json()
        .unwrap();

    let mut res = String::new();
    if output.code != 0 {
        for line in output.stderr {
            res.push_str(&line.text);
            res.push('\n');
        }
    } else {
        for line in output.asm {
            res.push_str(&line.text);
            res.push('\n');
        }
    }
    res
}

/// Send data to Compiler Explorer and shortens it. This may be used when the to be compiled sources are too large to fit into the URL.
/// Returns the shortened URL
pub fn shorten(client: Client, host: &str, src: String, compiler: &str, args: String) -> String {
    // Find language based on compiler.
    let compilers = get_compilers(&client.clone(), host, None);
    let language: String = compilers
        .iter()
        .find(|&x| x.id == compiler)
        .map_or("c++".to_string(), |c| c.lang.clone());

    let source = json!(
        { "sessions": [
            {
                "id": 1,
                "language": language,
                "source": src,
                "compilers": [{"id": &compiler,"options": args}]
            }
        ]}
    );

    let response = client
        .post(format!("{}/shortener", host).as_str())
        .json(&source)
        .send();

    let mut output_posted = match response {
        Ok(output_posted) => output_posted,
        Err(e) => return format!("Error sending: {}", e),
    };

    let output: Url = match output_posted.json() {
        Ok(output) => output,
        Err(e) => {
            return format!(
                "Error decoding result: {} {}",
                e,
                output_posted.text().unwrap()
            )
        }
    };
    output.url
}

