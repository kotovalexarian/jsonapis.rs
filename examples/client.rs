/*
 * This example uses fake JSON:API
 * https://jsonapiplayground.reyesoft.com
 */

use jsonapis::{Client, Document, Response};

fn main() {
    let client = Client::new("https://jsonapiplayground.reyesoft.com/v2");

    let response: Response = client
        .get("/stores", &[("filter[created_by]", "1,3".to_string())])
        .unwrap();

    let document: &Document = response.document();

    println!("{:#?}", document);
}
