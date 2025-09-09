use std::collections::HashMap;

use super::error::{ParseError};
use super::fields::{Author, Authors};


#[derive(Debug, PartialEq)]
enum BibtexEntryKind {
    Article, Book, Booklet, Conference, Inbook, InCollection, 
    InProceedings, Manual, MasterThesis, Misc, PhdThesis,
    Proceedings, TechReport, Unpublished, Unknown   
}

impl BibtexEntryKind {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "article"       => Self::Article, 
            "book"          => Self::Book, 
            "booklet"       => Self::Booklet,
            "conference"    => Self::InProceedings,
            "inbook"        => Self::Inbook,
            "incollection"  => Self::InCollection,
            "inproceedings" => Self::InProceedings,
            "manual"        => Self::Manual,
            "mastersthesis" => Self::MasterThesis,
            "misc"          => Self::Misc,
            "phdthesis"     => Self::PhdThesis,
            "proceedings"   => Self::Proceedings,
            "techreport"    => Self::TechReport,
            "unpublished"   => Self::Unpublished,
            _               => Self::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
struct BibtexEntry {
    /// The kind of entry
    pub kind: BibtexEntryKind, 
    /// The key for the entry, for example: Anderson2004
    pub citekey: String, 
    /// The fields of the entry
    pub fields: HashMap<String, String>
}

struct BibtexParser<'a> {
    /// The raw UTF-8 input slice from a BibTeX file
    input: &'a [u8],
    /// The cursor position 
    cursor: usize,
}

impl<'a> BibtexParser<'a> {
    fn new(input: &'a str) -> Self {
        BibtexParser { 
            input: input.as_bytes(), 
            cursor: 0 
        }
    }

    /// Parses the entire input slice
    pub fn parse(&mut self) -> Result<Vec<BibtexEntry>, ParseError> {
        self.consume_whitespace();
        
        if self.peek().is_none() {
            return Err(ParseError::EmptyBibliography);
        }
        
        let mut entries: Vec<BibtexEntry> = Vec::new();

        while self.peek().is_some() {
            let entry = self.parse_entry()?;
            entries.push(entry);
            self.consume_whitespace();
        }

        Ok(entries)
    }

    /// Peeks at the byte the cursor is currently pointing at 
    fn peek(&self) -> Option<u8> {
        self.input.get(self.cursor).copied()
    }

    /// Advances the cursor forward one
    fn advance(&mut self) {
        self.cursor += 1;
    }

    /// Parces a BibTeX entry
    fn parse_entry(&mut self) -> Result<BibtexEntry, ParseError> {
        self.consume_char(b'@')?; // jump to the next entry
        let entry_kind_str = self.consume_identifier();
        let entry_kind = BibtexEntryKind::from_str(&entry_kind_str)
            .ok_or(ParseError::InvalidEntryType{ recieved: entry_kind_str })?;

            
        self.consume_char(b'{')?; // jump inside the braces
        let citekey = self.consume_identifier();
        self.consume_char(b',')?;

        let mut fields = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.peek() == Some(b'}') {
                // we are at the end of the entry
                break;
            }

            let (key, value) = self.parse_field()?;
            fields.insert(key, value);

            self.consume_whitespace();
            if self.peek() == Some(b',') {
                self.advance();
            } else if self.peek() != Some(b'}') {
                let recieved = self.peek().map(|b| b as char).unwrap_or(' ');
                return Err(ParseError::UnexpectedCharacter { expected: '}', recieved })
            }
        }

        self.consume_char(b'}')?;

        let entry = BibtexEntry { 
            kind: entry_kind, 
            citekey, 
            fields, 
        };
        Ok(entry)
    }

    /// Parses a BibTeX field
    fn parse_field(&mut self) -> Result<(String, String), ParseError> {
        let key = self.consume_identifier();
        self.consume_char(b'=')?;
        let value = self.parse_value()?;
        Ok((key, value))
    }

    /// Parses a the value of some field 
    fn parse_value(&mut self) -> Result<String, ParseError> {
        self.consume_whitespace();
        match self.peek() {
            Some(b'{') => self.consume_braced_string(),
            Some(b'"') => self.consume_quoted_string(), 
            Some(c) if c.is_ascii_digit() => Ok(self.consume_identifier()), 
            _ => Err(ParseError::InvalidValue), 
        }
    }

    /// Parses the author/s
    fn parse_authors(&mut self) -> Result<Authors, ParseError> {
        todo!()
    }

    /// Parses the date
    fn parse_date(&mut self) -> Result<Date, ParseError> {
        todo!()
    }

    /// Consume a braced string
    fn consume_braced_string(&mut self) -> Result<String, ParseError> {
        self.consume_char(b'{')?;
        let start = self.cursor;
        let mut brace_level = 1;

        while let Some(byte) = self.peek() {
            if byte == b'{' {
                brace_level += 1;
            } else if byte == b'}' {
                brace_level -= 1;
                if brace_level == 0 {
                    break;
                }
            }
            self.advance();
        }

        if brace_level != 0 {
            return Err(ParseError::UnexpectedEOF);
        }

        let value = String::from_utf8_lossy(&self.input[start..self.cursor]).to_string();
        self.advance();
        Ok(value)
    }

    fn consume_quoted_string(&mut self) -> Result<String, ParseError> {
        self.consume_char(b'"')?;
        let start = self.cursor;

        while let Some(byte) = self.peek() {
            if byte == b'"' {
                break;
            }
            self.advance();
        }

        if self.peek() != Some(b'"') {
            return Err(ParseError::UnexpectedEOF);
        }

        let value = String::from_utf8_lossy(&self.input[start..self.cursor]).to_string();
        self.advance();
        Ok(value)
    }

    fn consume_char(&mut self, expected: u8) -> Result<(), ParseError> {
        self.consume_whitespace();
        
        match self.peek() {
            Some(byte) if byte == expected => {
                self.advance();
                Ok(())
            }
            Some(other_byte) => {
                return Err(ParseError::UnexpectedCharacter { 
                    expected: expected as char, 
                    recieved: other_byte as char,  
                })
            }
            None => Err(ParseError::UnexpectedEOF)
        }
    }

    /// Consumes everything until we reach a certain character, returning a String of everything consumed
    fn consume_identifier(&mut self) -> String {
        let start = self.cursor;

        while let Some(byte) = self.peek() {
            if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b':' || byte == b'_' {
                self.advance();
            } else {
                break;
            }
        }

        String::from_utf8_lossy(&self.input[start..self.cursor]).to_string()
    }

    /// Consumes whitespace characters as well as inline comments
    fn consume_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(b'%') => {
                    while let Some(byte) = self.peek() {
                        if byte == b'\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                Some(byte) if byte.is_ascii_whitespace() => {
                    self.advance();
                }
                _ => {
                    // neither a comment or whitespace
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_article() {
        let input = r#"
            @article{test_key,
                author = {Author, A.},
                title = "A Test Title", % This is an inline comment
                year = 2025,
                journal = "Journal of Tests", 
            }
        "#;

        let mut parser = BibtexParser::new(input);
        let result = parser.parse().unwrap();
        
        assert_eq!(result.len(), 1);
        let entry = &result[0];

        assert_eq!(entry.kind, BibtexEntryKind::Article);
        assert_eq!(entry.citekey, "test_key");
        assert_eq!(entry.fields.get("author").unwrap(), "Author, A.");
        assert_eq!(entry.fields.get("title").unwrap(), "A Test Title");
        assert_eq!(entry.fields.get("year").unwrap(), "2025");
        assert_eq!(entry.fields.get("journal").unwrap(), "Journal of Tests");
    }

    #[test]
    fn test_multiple_entries() {
        let input = r#"
            @article{key1, title = "Title 1"}
            @book{key2, title = "Title 2", author={Author B}}
        "#;
        let mut parser = BibtexParser::new(input);
        let result = parser.parse().unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].citekey, "key1");
        assert_eq!(result[0].kind, BibtexEntryKind::Article);
        assert_eq!(result[1].citekey, "key2");
        assert_eq!(result[1].kind, BibtexEntryKind::Book);
        assert_eq!(result[1].fields.get("author").unwrap(), "Author B");
    }

    #[test]
    fn test_nested_braces() {
         let input = r#"
            @misc{nested,
                title = {A Title with {Nested Braces} is Cool},
            }
        "#;
        let mut parser = BibtexParser::new(input);
        let result = parser.parse().unwrap();
        
        assert_eq!(result.len(), 1);
        let entry = &result[0];
        assert_eq!(entry.fields.get("title").unwrap(), "A Title with {Nested Braces} is Cool");
    }
}