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

#[derive(Eq, PartialEq, Debug)]
enum Lexem {
    OpenTagOpenBracket,   // <
    CloseTagOpenBracket,  // </
    CloseTagCloseBracket, // >
    Equal,                // =
    Name(String),         // tag or attribute name
    Value(String),        // attribute's value
    Text(String),         // text with normalized spaces
}

fn read_lexem(buffer: &[u8], offset: usize) -> Result<(Lexem, usize), String> {
    if buffer[offset] == b'<' {
        if offset + 1 < buffer.len() && buffer[offset + 1] == b'/' {
            return Ok((Lexem::CloseTagOpenBracket, offset + 2));
        } else if offset + 3 < buffer.len()
            && buffer[offset + 1] == b'!'
            && buffer[offset + 2] == b'-'
            && buffer[offset + 3] == b'-'
        {
            return Err("Not implemented comment".to_string());
        } else {
            return Ok((Lexem::OpenTagOpenBracket, offset + 1));
        }
    } else {
        return Err(format!("unsupported char {}", buffer[offset]));
    }
}

#[cfg(test)]
mod read_lexem_should {
    use crate::{read_lexem, Lexem};
    #[test]
    fn return_open_tag() {
        let actual = read_lexem(b"<a href='b'>", 0);
        assert_eq!(Ok((Lexem::OpenTagOpenBracket, 1usize)), actual);
    }
}

fn main() {
    println!("Hello, world!");
}
