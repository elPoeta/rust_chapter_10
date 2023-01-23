#![allow(unused)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    {
        // fails because string1 is longer than string2
        /*
            let string1 = String::from("long string is long");
            let result;
            {
                let string2 = String::from("xyz");
                result = longest(string1.as_str(), string2.as_str());
            }
            println!("The longest string is {}", result);
        */
    }

    {
        #[derive(Debug)]
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{:#?}", i);
    }
    {
        // static lifetime
        let s: &'static str = "I have a static lifetime.";
    }
    {
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}

//generic lifetime parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
