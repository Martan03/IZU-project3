use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::object::Object;

pub struct Node {
    pub name: String,
    pub label: String,
    pub trans: String,
    pub children: Vec<Node>,
}

impl Node {
    /// Creates new leaf node
    pub fn leaf(name: String, label: String) -> Self {
        Self {
            name,
            label,
            trans: String::new(),
            children: vec![],
        }
    }

    /// Adds child to the node and transition
    pub fn add_child(
        &mut self,
        mut child: Node,
        val: &String,
        mp: &[&Object],
    ) {
        child.trans = format!(
            "{} {{{}}}",
            val,
            mp.iter()
                .map(|o| o.id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        self.children.push(child);
    }

    /// Prints the tree for graph generating
    pub fn graph_write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.children.is_empty() {
            writer.write_all(
                format!(
                    "    {} [shape=box, style=rounded, label=\"{}\"]\n",
                    self.name, self.label
                )
                .as_bytes(),
            )?;
            return Ok(());
        }

        writer.write_all(
            format!(
                "    {} [shape=record, label=\"{}\"]\n",
                self.name, self.label
            )
            .as_bytes(),
        )?;
        for child in self.children.iter() {
            writer.write_all(
                format!(
                    "    {} -> {} [label=\"{}\"]\n",
                    self.name, child.name, child.trans
                )
                .as_bytes(),
            )?;
            child.graph_write(writer)?;
        }
        Ok(())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} [label=\"{}\"]", self.name, self.label)?;

        if self.children.is_empty() {
            return Ok(());
        }

        for child in self.children.iter() {
            write!(
                f,
                "{} -> {} [label=\"{}\"]\n{}",
                self.name, child.name, child.trans, child
            )?;
        }
        Ok(())
    }
}
