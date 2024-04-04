use parser::{Parser, ParserErr};

mod object;
mod parser;

fn main() -> Result<(), ParserErr> {
    let parser = Parser::parse("model-xsleza26.txt")?;

    println!("Attributes:");
    for item in parser.attr {
        println!("{}", item.0);
    }

    println!("\nClasses:");
    for class in parser.class {
        println!("{}", class);
    }

    println!("\nObjects:");
    for item in parser.object {
        println!("{}, {}", item.id, item.class);
    }

    Ok(())
}

fn id3(mp: Vec<String>, mv: Vec<String>) {
    if mp.iter().all(|x| *x == mp[0]) {
        println!("{}", mp[0]);
        return;
    }

    if mv.is_empty() {
        println!("Disjunkce");
        return;
    }

    let attr = highest_entropy(&mp, &mv);
    // Remove attr from `mv`
}

fn highest_entropy(mp: &Vec<String>, mv: &Vec<String>) -> String {
    "IDK".to_string()
}

fn entropy() -> f64 {
    0.0
}
