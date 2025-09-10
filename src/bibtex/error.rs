#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedCharacter(char), 
    InvalidEntryKind(String), 
    EmptyBibliography,
    UnexpectedEOF, 
    InvalidValue, 
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter(c) => {
                write!(f, "unexpected character '{}'", c)
            }
            Self::InvalidEntryKind(s) => {
                write!(f, "invalid entry type '{}'", s)
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