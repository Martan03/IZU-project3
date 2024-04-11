use std::fmt::Display;

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

    /// Prints the tree for graph generating
    pub fn graph_write(&self) {
        if self.children.is_empty() {
            println!(
                "    {} [shape=box, style=rounded, label=\"{}\"]",
                self.name, self.label
            );
            return;
        }

        println!("    {} [shape=record, label=\"{}\"]", self.name, self.label);
        for child in self.children.iter() {
            println!(
                "    {} -> {} [label=\"{}\"]",
                self.name, child.name, child.trans
            );
            child.graph_write();
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [label=\"{}\"]\n", self.name, self.label)?;

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
