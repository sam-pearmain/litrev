use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedCharacter { expected: char, recieved: char } , 
    InvalidEntryType { recieved: String }, 
    EmptyBibliography,
    UnexpectedEOF, 
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter { expected, recieved } => {
                write!(f, "unexpected character '{}'. expected '{}'", recieved, expected)
            }
            Self::InvalidEntryType { recieved } => {
                write!(f, "invalid entry type '{}'", recieved)
            }
            Self::EmptyBibliography => {
                write!(f, "failed to parse BibTeX file, seemingly empty")
            }
            Self::UnexpectedEOF => {
                write!(f, "unexpected eof")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum BibtexEntryKind {
    Article, 
    Book,
    Booklet, 
    Conference, 
    Inbook, 
    InCollection, 
    InProceedings, 
    Manual, 
    MasterThesis, 
    Misc, 
    PhdThesis,
    Proceedings, 
    TechReport, 
    Unpublished,  
}

impl BibtexEntryKind {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "article"       => Some(Self::Article), 
            "book"          => Some(Self::Book), 
            "booklet"       => Some(Self::Booklet),
            "conference"    => Some(Self::InProceedings),
            "inbook"        => Some(Self::Inbook),
            "incollection"  => Some(Self::InCollection),
            "inproceedings" => Some(Self::InProceedings),
            "manual"        => Some(Self::Manual),
            "mastersthesis" => Some(Self::MasterThesis),
            "misc"          => Some(Self::Misc),
            "phdthesis"     => Some(Self::PhdThesis),
            "proceedings"   => Some(Self::Proceedings),
            "techreport"    => Some(Self::TechReport),
            "unpublished"   => Some(Self::Unpublished),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct BibtexEntry {
    pub kind: BibtexEntryKind, 
    pub citekey: String, 
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
        self.consume_char(b'@')?; 
        let entry_type_str = self.consume_identifier();
        let entry_type = BibtexEntryKind::from_str(&entry_type_str)
            .ok_or(ParseError::InvalidEntryType{ recieved: entry_type_str })?;
        todo!()
    }

    /// Parces a BibTeX field
    fn parse_field(&mut self) -> (String, String) {
        todo!()
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

    /// Consumes whitespace characters
    fn consume_whitespace(&mut self) {
        while let Some(byte) = self.peek() {
            if byte.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Consumes until a certain character is reached
    fn consume_until(&mut self, character: u8) -> Result<(), ParseError> {
        while let Some(byte) = self.peek() {
            if byte == character {
                self.advance();
                return Ok(());
            }
        }
        Err(ParseError::UnexpectedEOF)
    }

    /// Go to the next line
    fn next_line(&mut self) -> Result<(), ParseError> {
        self.consume_until(b'\n')
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(b' '.is_ascii_alphanumeric(), false);
    }
}