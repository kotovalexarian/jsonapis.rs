use jsonapis::Builder as JsonApisBuilder;
use serde_json::{json, Value};

fn main() {
    let document = jsonapis::DocumentBuilder::default()
        .jsonapi(jsonapis::Version::new(0))
        .meta1("current_page", 1)
        .meta1("items_per_page", 2)
        .meta1("total_pages", 3)
        .meta1("total_items", 6)
        .link("self", "http://example.com/posts.json?page=1")
        .link("first", "http://example.com/posts.json?page=1")
        .link("last", "http://example.com/posts.json?page=3")
        .link("next", "http://example.com/posts.json?page=2")
        .data(vec![
            jsonapis::ResourceBuilder::new_with_id("posts", "1")
                .link("self", "http://example.com/posts/1.json")
                .attr("title", "Some blog post")
                .attr("summary", "Here is the beginning of some blog post.")
                .rel(
                    "author",
                    jsonapis::ResourceBuilder::new_with_id("users", "1")
                        .link("self", "http://example.com/users/1.json")
                        .attr("username", "alice"),
                ),
            jsonapis::ResourceBuilder::new_with_id("posts", "2")
                .link("self", "http://example.com/posts/2.json")
                .attr("title", "Other blog post")
                .attr("summary", "Here is the beginning of other blog post.")
                .rel(
                    "author",
                    jsonapis::ResourceBuilder::new_with_id("users", "2")
                        .link("self", "http://example.com/users/2.json")
                        .attr("username", "bob"),
                ),
        ])
        .unwrap();

    let expected_value = json!({
        "jsonapi": json!({
            "version": json!("1.0"),
            "meta": json!(null),
        }),
        "meta": json!({
            "current_page": json!(1),
            "items_per_page": json!(2),
            "total_pages": json!(3),
            "total_items": json!(6),
        }),
        "links": json!({
            "self": json!("http://example.com/posts.json?page=1"),
            "related": json!(null),
            "first": json!("http://example.com/posts.json?page=1"),
            "last": json!("http://example.com/posts.json?page=3"),
            "prev": json!(null),
            "next": json!("http://example.com/posts.json?page=2"),
            "about": json!(null),
        }),
        "data": json!([
            json!({
                "type": json!("posts"),
                "id": json!("1"),
                "meta": json!(null),
                "links": json!({
                    "self": json!("http://example.com/posts/1.json"),
                    "related": json!(null),
                    "first": json!(null),
                    "last": json!(null),
                    "prev": json!(null),
                    "next": json!(null),
                    "about": json!(null),
                }),
                "attributes": json!({
                    "title": json!("Some blog post"),
                    "summary": json!("Here is the beginning of some blog post."),
                }),
                "relationships": json!({
                    "author": json!({
                        "meta": json!(null),
                        "links": json!(null),
                        "data": json!({
                            "type": json!("users"),
                            "id": json!("1"),
                            "meta": json!(null),
                            "links": json!({
                                "self": json!("http://example.com/users/1.json"),
                                "related": json!(null),
                                "first": json!(null),
                                "last": json!(null),
                                "prev": json!(null),
                                "next": json!(null),
                                "about": json!(null),
                            }),
                            "attributes": json!({
                                "username": json!("alice"),
                            }),
                            "relationships": json!(null),
                        }),
                    }),
                }),
            }),
            json!({
                "type": json!("posts"),
                "id": json!("2"),
                "meta": json!(null),
                "links": json!({
                    "self": json!("http://example.com/posts/2.json"),
                    "related": json!(null),
                    "first": json!(null),
                    "last": json!(null),
                    "prev": json!(null),
                    "next": json!(null),
                    "about": json!(null),
                }),
                "attributes": json!({
                    "title": json!("Other blog post"),
                    "summary": json!("Here is the beginning of other blog post."),
                }),
                "relationships": json!({
                    "author": json!({
                        "meta": json!(null),
                        "links": json!(null),
                        "data": json!({
                            "type": json!("users"),
                            "id": json!("2"),
                            "meta": json!(null),
                            "links": json!({
                                "self": json!("http://example.com/users/2.json"),
                                "related": json!(null),
                                "first": json!(null),
                                "last": json!(null),
                                "prev": json!(null),
                                "next": json!(null),
                                "about": json!(null),
                            }),
                            "attributes": json!({
                                "username": json!("bob"),
                            }),
                            "relationships": json!(null),
                        }),
                    }),
                }),
            }),
        ]),
        "errors": json!(null),
    });

    let actual_json = serde_json::to_string(&document).unwrap();

    let actual_value: Value = serde_json::from_str(&actual_json).unwrap();

    println!("{:#?}", actual_value);

    assert_eq!(actual_value, expected_value);
}
