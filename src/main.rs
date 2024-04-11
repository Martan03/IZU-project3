use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

use id3::id3;
use node::Node;
use object::{Attr, Object};
use parser::Parser;

mod id3;
mod node;
mod object;
mod parser;

fn main() -> Result<(), String> {
    let xlogin = "xsleza26";
    let model = format!("model-{}.txt", xlogin);
    let md = format!("{}.md", xlogin);
    let dot = format!("{}.dot", xlogin);

    let parser = Parser::parse(&model).map_err(|e| e.to_string())?;

    let objects: Vec<&Object> = parser.object.iter().collect();
    let attrs: Vec<&Attr> = parser.attr.iter().collect();
    let mut id = 1;

    let node = id3(&mut id, &objects, &attrs);

    let mut md_out = File::create(md).map_err(|e| e.to_string())?;
    md_out
        .write_all(format!("{node}").as_bytes())
        .map_err(|e| e.to_string())?;

    let mut dot_out = File::create(dot).map_err(|e| e.to_string())?;
    get_dot(&mut dot_out, &node).map_err(|e| e.to_string())?;

    let status = Command::new("dot")
        .args([
            "-Tpdf",
            &format!("-o{}.pdf", xlogin),
            &format!("{}.dot", xlogin),
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        println!("Failed to generate graphviz file");
    }

    Ok(())
}

fn get_dot<W: Write>(writer: &mut W, node: &Node) -> io::Result<()> {
    writer.write_all("digraph {\n".as_bytes())?;
    node.graph_write(writer)?;
    writer.write_all("}\n".as_bytes())
}
