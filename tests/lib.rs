extern crate readability;
extern crate url;

use std::fs::File;
use url::Url;

#[test]
fn test_extract_title() {
    let mut file = File::open("./data/title.html").unwrap();
    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(&mut file, &url).unwrap();
    assert_eq!(product.title, "This is title");
}

#[test]
fn test_fix_rel_links() {
    let mut file = File::open("./data/rel.html").unwrap();
    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(&mut file, &url).unwrap();
    assert_eq!(product.content, "<!DOCTYPE html><html><head><title>This is title</title></head><body><p><a href=\"https://example.com/poop\"> poop </a></p></body></html>");
}

#[test]
fn test_fix_img_links() {
    let mut file = File::open("./data/img.html").unwrap();
    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(&mut file, &url).unwrap();
    assert_eq!(product.content, "<!DOCTYPE html><html><head><title>This is title</title></head><body><p><img src=\"https://example.com/poop.png\"></p></body></html>");
}

#[test]
fn test_extract_text() {
    // previous result: better known asMuddy Waterswas an Americanbluessinger-songwriterand musician who was an important figure
    // new result:  better known as Muddy Waters was an American blues singer-songwriter and musician who was an important figure
    let mut file = File::open("./data/muddy_waters.html").unwrap();
    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(&mut file, &url).unwrap();
    println!("{}", product.text);
    let expected = r#"McKinley Morganfield (April 4, 1914 – April 30, 1983),better known as Muddy Waters was an American blues singer-songwriter and musician who was an important figure in the post-World War II blues
    scene, and is often cited as the "father of modern Chicago blues ".His style of playing has been described as "raining down Delta beatitude"."#;
    assert_eq!(expected, product.text);
}
