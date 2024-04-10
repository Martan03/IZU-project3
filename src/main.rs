use std::collections::HashMap;

use object::{Attr, Object};
use parser::{Parser, ParserErr};

mod object;
mod parser;

fn main() -> Result<(), ParserErr> {
    let parser = Parser::parse("model-xsleza26.txt")?;

    let objects: Vec<&Object> = parser.object.iter().collect();
    let cnt = class_cnt(&objects);

    let mut emp = 0.0;
    for (_, val) in cnt {
        emp += entropy(val, parser.object.len());
    }
    println!("{emp}");

    for (id, attr) in parser.attr.iter().enumerate() {
        let e = attr_entropy(emp, id, &attr.values, &objects);
        println!("{e}");
    }

    Ok(())
}
/*
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

fn highest_entropy(mp: &Vec<String>, mv: &Vec<Attr>) -> String {
    for (id, attr) in mv.iter().enumerate() {
        for value in attr.attr {

        }
    }
    "IDK".to_string()
}

fn attr_entropy(mp: &Vec<String>, values: &Vec<String>) -> f64 {
    let mut sum = 0.0;
    for value in values {
        let obj = mp.iter().filter(|&x| )
    }
    0.0
}*/
fn class_cnt(objects: &Vec<&Object>) -> HashMap<String, usize> {
    let mut cnt: HashMap<String, usize> = HashMap::new();
    for obj in objects {
        cnt.entry(obj.class.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    cnt
}

fn attr_entropy(
    emp: f64,
    id: usize,
    vals: &Vec<String>,
    objs: &Vec<&Object>,
) -> f64 {
    let mut e = emp;
    for attr in vals {
        let obj = objs
            .iter()
            .filter(|x| x.attr[id] == *attr)
            .map(|v| *v)
            .collect();

        let cnt = class_cnt(&obj);
        e -= (obj.len() as f64 / objs.len() as f64)
            * cnt.values().map(|v| entropy(*v, obj.len())).sum::<f64>();
    }
    e
}

fn entropy(dividend: usize, divisor: usize) -> f64 {
    let frac = dividend as f64 / divisor as f64;
    -frac * f64::log2(frac)
}
