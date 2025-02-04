use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::object::{Attr, Object};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserErr {
    ReadErr,
    NotEndedBlock,
    UnexpectedFormat,
    InvalidClassName(String),
    InvalidOrder,
    InvalidObject,
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErr::ReadErr => write!(f, "error reading file"),
            ParserErr::NotEndedBlock => write!(f, "block not ended"),
            ParserErr::UnexpectedFormat => write!(f, "unexpected file format"),
            ParserErr::InvalidClassName(class) => {
                write!(f, "invalid class name '{}'", class)
            }
            ParserErr::InvalidOrder => {
                write!(f, "objects before attributes and classes")
            }
            ParserErr::InvalidObject => write!(f, "invalid object format"),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub attr: Vec<Attr>,
    pub class: Vec<String>,
    pub object: Vec<Object>,
}

impl Parser {
    pub fn parse(filename: &str) -> Result<Self, ParserErr> {
        let mut parser = Self {
            attr: vec![],
            class: vec![],
            object: vec![],
        };

        let file = File::open(filename).map_err(|_| ParserErr::ReadErr)?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines().map_while(std::io::Result::ok);
        while let Some(line) = lines.next() {
            match line.trim() {
                "attributes {" => {
                    parser.parse_block(&mut lines, Parser::parse_attr)?
                }
                "classes {" => {
                    parser.parse_block(&mut lines, Parser::parse_class)?
                }
                "objects {" => {
                    parser.parse_block(&mut lines, Parser::parse_object)?
                }
                "" => {}
                _ => {
                    return Err(ParserErr::UnexpectedFormat);
                }
            }
        }

        Ok(parser)
    }

    fn parse_block<T, F>(
        &mut self,
        lines: &mut T,
        parse_line: F,
    ) -> Result<(), ParserErr>
    where
        T: Iterator<Item = String>,
        F: Fn(&mut Parser, &str) -> Result<(), ParserErr>,
    {
        for line in lines.by_ref() {
            let trim = line.trim();
            if trim == "}" {
                return Ok(());
            }

            if trim.is_empty() {
                continue;
            }

            parse_line(self, trim)?;
        }
        Err(ParserErr::NotEndedBlock)
    }

    /// Parses attributes
    fn parse_attr(&mut self, line: &str) -> Result<(), ParserErr> {
        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(ParserErr::UnexpectedFormat);
        }

        let values =
            parts[1].split_whitespace().map(|s| s.to_string()).collect();

        self.attr.push(Attr {
            id: self.attr.len(),
            name: parts[0].to_string(),
            values,
        });
        Ok(())
    }

    /// Parses classes
    fn parse_class(&mut self, line: &str) -> Result<(), ParserErr> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 1 {
            return Err(ParserErr::InvalidClassName(line.to_string()));
        }

        self.class.push(line.to_string());
        Ok(())
    }

    fn parse_object(&mut self, line: &str) -> Result<(), ParserErr> {
        if self.attr.is_empty() || self.class.is_empty() {
            return Err(ParserErr::InvalidOrder);
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 + self.attr.len() {
            return Err(ParserErr::InvalidObject);
        }

        let id = parts[0]
            .parse::<usize>()
            .map_err(|_| ParserErr::InvalidObject)?;
        let attr: Vec<String> =
            parts[2..].iter().map(|s| s.to_string()).collect();
        self.object.push(Object::new(id, parts[1], attr));

        Ok(())
    }
}
