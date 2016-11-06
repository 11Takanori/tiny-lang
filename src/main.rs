extern crate regex;

use std::str;
use regex::Regex;

fn main() {
    let code = format!(r#"
        if 1:
          if 2:
            print("...")
            if false:
              pass
            print("done!")
          2

        print "The End"
      "#
    );
    tokenize(code);
}

fn tokenize(code: String) {
    let keywords = vec!["def", "class", "if", "true", "false", "nil"];
    let low_re = Regex::new(r"\A([a-z]\w*)").unwrap();
    let up_re = Regex::new(r"\A([A-Z]\w*)").unwrap();
    let num_re = Regex::new(r"\A(\d+)").unwrap();
    // let string_re = Regex::new(r"^\*\w*").unwrap();
    let indent_re = Regex::new(r"\A\n( *)").unwrap();


    let mut tokens: Vec<&str> = vec![];
    let current_indent = 0;
    let indent_stack: Vec<i32> = vec![];
    let mut i = 0;

    while i < code.len() {         
        unsafe {
            let mut chunk = code.slice_unchecked(i, code.len());

            for caps in low_re.captures_iter(&chunk) {
                if keywords.contains(&caps.at(1).unwrap()) {
                    tokens.push(caps.at(1).unwrap())
                }
                i +=caps.at(1).unwrap().len(); 
            } 
            
            for caps in up_re.captures_iter(&chunk) {
                tokens.push(caps.at(1).unwrap());
                i +=caps.at(1).unwrap().len(); 
            }

            for caps in num_re.captures_iter(&chunk) {
                tokens.push(caps.at(1).unwrap());
                i +=caps.at(1).unwrap().len(); 
            }

            // for caps in string_re.captures_iter(&chunk) {
            //     tokens.push(caps.at(1).unwrap());
            //     i +=caps.at(1).unwrap().len() + 2; 
            // }

            for caps in indent_re.captures_iter(&chunk) {
                tokens.push(caps.at(1).unwrap());
                i +=caps.at(1).unwrap().len(); 
            }
        };
        i = i + 1; 
    }
}