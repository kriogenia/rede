# Rede Parser

Library crate to receive the content of Rede's files and generate the requests or environments
to use in the command-line binary. It can be used on its own to implement Rede's format in other
projects or implementations.

## Usage

The library offers the function `rede_parser::parse_request` to convert a given string into a valid
`rede_parser::Request`.

```rust
let toml = r#"
[http]
method = "POST"
url = "http://localhost:8080/note"

[headers]
Content-Type = "application/json"

[body]
raw = """
{
  "title": "Implement rede_parser" ,
  "description": "Implement it following the example
}
"""
"#;

let request = rede_parser::parse_request(toml)?;
assert_eq!(request.method, Method::POST);
assert_eq!(request.url, "http://localhost:8080/note");
assert_eq!(request.headers["Content-Type"], "application/json");
if let Body::Raw { content, mime } = &request.body {
  assert_eq!(mime, &"text/plain; charset=utf-8");
  println!("{}", &request.body);
}
```
