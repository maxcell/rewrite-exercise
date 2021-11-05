use std::error::Error;

use lol_html::{HtmlRewriter, Settings, element};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("index.html"); 
    let output = process_input(input);
    println!("{}", output?);

    Ok(())
}

// Second problem we pivoted:
// Instead of doing all the work with unwraps:
// How can we make sure to do this with a function and idiomatically handle
// the errors along the way
fn process_input(input: &str) -> Result<String, Box<dyn Error>> {
  let mut output = vec![];

    let mut rewriter = HtmlRewriter::try_new(
        Settings {
            element_content_handlers: vec![
                element!("img[src]", |el| { 
                    el.set_attribute("loading", "lazy")?;

                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    )?;
    rewriter.write(input.as_bytes())?;
    rewriter.end()?;

   Ok(String::from_utf8(output)?)
}