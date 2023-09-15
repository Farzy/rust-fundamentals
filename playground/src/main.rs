#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    // infinite(0);
    sentences();
}

/*
    Test stack space. The mode local variables are added to tne "infinite"
    function, the less recursion and stack creation will be supported before the
    program crashes.
*/
#[allow(unconditional_recursion)]
fn infinite(limit: u128) -> ! {
    let _a: u128 = 42; // Try commenting in/out these lines and re-run the program
    let _b: u128 = 22; // Try commenting in/out these lines and re-run the program
    let _c: u128 = 11; // Try commenting in/out these lines and re-run the program
                       // let _d: [u128; 100000]; // Try commenting in/out these lines and re-run the program

    // if limit > 128 {
    //     return;
    // }
    println!("loop {}!", limit);
    infinite(limit + 1);
}

// Code from https://blog.logrocket.com/understanding-lifetimes-in-rust/

#[derive(Debug)]
struct S<'a> {
    first: &'a str,
    last: &'a str,
}

fn try_create(paragraph: &str) -> Option<S> {
    let mut sentences = paragraph.split('.').filter(|s| !s.is_empty());
    match (sentences.next(), sentences.next_back()) {
        (Some(first), Some(last)) => Some(S { first, last }),
        (Some(first), None) => Some(S { first, last: first }),
        _ => None,
    }
}

fn sentences() {
    let paragraphs = [
        "For example, let’s say you want to find the first and the last sentence of a paragraph and keep them in a struct S. Because you don’t want to copy the data, you need to use references and give them lifetime annotations. You could use a function like this to populate the struct. For simplicity’s sake, we’ll assume that a full stop is the only sentence-ending punctuation mark in use. If the paragraph is empty, return None, and if there is only a single sentence, use that as both the first and the last sentence.",
        "This is a single sentence with no dot",
        ""
        ];
    for (i, paragraph) in paragraphs.iter().enumerate() {
        println!("* Paragraph number {}:", i + 1);
        if let Some(S { first, last }) = try_create(paragraph) {
            println!("First Sentence: “{}“.", first);
            println!("Last Sentence: “{}“.", last);
        } else {
            println!("No sentence found");
        }
    }
}
