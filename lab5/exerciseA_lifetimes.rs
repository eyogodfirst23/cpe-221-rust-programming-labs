fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn summarise(&self) -> &str {
        &self.content[..self.content.len().min(80)]
    }
}

fn first_sentence<'a>(text: &'a str) -> &'a str {
    match text.find('.') {
        Some(idx) => &text[..idx],
        None => text,
    }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);
    }

    let article = String::from(
        "Rust 2024 edition brings many improvements. It has a new borrow checker.",
    );
    let imp = Important { content: &article };
    println!("Summary: {}", imp.summarise());

    println!("First sentence: {}", first_sentence(&article));
    println!(
        "First sentence (no dot): {}",
        first_sentence("no punctuation here")
    );
}
