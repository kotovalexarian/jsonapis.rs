#[cfg(feature = "client")]
use jsonapis::{Client, Document, Response};

#[cfg(not(feature = "client"))]
fn main() {}

#[cfg(feature = "client")]
fn main() {
    // This example uses fake JSON:API
    let client = Client::new("https://jsonapiplayground.reyesoft.com/v2");

    let response: Response = client
        .get("/stores", &[("filter[created_by]", "1,3".to_string())])
        .unwrap();

    let document: &Document = response.document();

    println!("{:#?}", document);
}
