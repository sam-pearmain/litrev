#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedCharacter { expected: char, recieved: char } , 
    InvalidEntryType { recieved: String }, 
    EmptyBibliography,
    UnexpectedEOF, 
    InvalidValue, 
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
            Self::InvalidValue => {
                write!(f, "invalid value within one of the entry fields given")
            }
        }
    }
}