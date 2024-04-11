use id3::id3;
use object::{Attr, Object};
use parser::{Parser, ParserErr};

mod id3;
mod node;
mod object;
mod parser;

fn main() -> Result<(), ParserErr> {
    let parser = Parser::parse("model-xsleza26.txt")?;

    let objects: Vec<&Object> = parser.object.iter().collect();
    let attrs: Vec<&Attr> = parser.attr.iter().collect();
    let mut id = 1;

    let node = id3(&mut id, &objects, &attrs);

    // .dot output
    // println!("{node}");

    // graph output
    println!("digraph {{");
    node.graph_write();
    println!("}}");

    Ok(())
}
