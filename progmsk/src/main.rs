/*

document = node+

node = text | openTag | closeTag | comment

openTag = '<' name attribute* '>'

closeTag = '</' name '>'

comment = '<!--' node* '-->'

attribute = name ['=' value]

value = quotelessString | string

string = '"' [^"]* '"'
       | "'" [^']* "'"

*/

enum Lexem {
    OpenTagOpenBracket,     // <
    CloseTagOpenBracket,    // </
    CloseTagCloseBracket,   // >
    Equal,                  // =
    Name(String),           // tag or attribute name
    Value(String),          // attribute's value
    Text(String)            // text with normalized spaces
}

fn read_lexem(buffer: &str, offset: usize) -> Result<(Lexem, usize), &str> {
    if buffer[offset] == '<' {
        if offset + 1 < buffer.length && buffer[offset + 1] == '/' {
            return Ok(Lexem::CloseTagOpenBracket, offset + 2);
        }
        else if offset + 3 < buffer.length && buffer[offset + 1] == '!'
        && buffer[offset + 2] == '-' && buffer[offset + 3] == '-' {
            return Err("Not implemented comment");
        }
        else {
            return Ok(Lexem::OpenTagOpenBracket, offset + 1);
        }
    }
}

#[cfg(test)]
mod read_lexem_should {
    fn return_open_tag() {
        let actual = read_lexem("<a href='b'>", 0);

        assert_eq!(Ok(Lexem::OpenTagOpenBracket, 1), actual);
    }
}

fn main() {
    println!("Hello, world!");
}
