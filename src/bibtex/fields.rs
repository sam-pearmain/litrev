

#[derive(Debug, PartialEq)]
pub struct Author {
    forename: String, 
    surname: String, 
    suffix: Option<String>,
    prefix: Option<String>,  
    is_lead_author: bool, 
}

#[derive(Debug)]
pub struct Authors {
    authors: Vec<Author>, 
}

impl Author {
    fn parse() -> Self {
        
    }
}

#[derive(Debug, PartialEq)]
pub struct PageRange {
    start: i32, 
    end: Option<i32>, // if we just have one page then no end page is given 
}

#[derive(Debug)]
pub struct Pages {
    pages: Vec<PageRange>
}