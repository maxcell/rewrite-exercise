use lol_html::{HtmlRewriter, Settings, element};

fn main() {
    // Initial Exercise:
    // Using lol_html, find all image tags and an attribute of
    // loading with the value "lazy".
    let input = include_str!("index.html"); 
    
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::try_new(
        Settings {
            element_content_handlers: vec![
                element!("img[src]", |el| { 
                    el.set_attribute("loading", "lazy").unwrap();

                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    ).unwrap();

    // Problem: we want to take from a &str -> &[u8]
    // Rust Strings are always a UTF-8 sequence
    // Solution: .as_bytes()
    rewriter.write(input.as_bytes()).unwrap();
    rewriter.end().unwrap();

    // An attempt: we thought we could turn our &str into &[u8] with try_into
    // let output: String = // (&output[..]).try_into().unwrap();
    println!("{}", String::from_utf8(output).unwrap());

    // Q: Why would we need to have .end() for our streaming rewriter?
}