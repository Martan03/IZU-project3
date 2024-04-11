use std::collections::HashMap;

use object::{Attr, Object};
use parser::{Parser, ParserErr};

mod object;
mod parser;

fn main() -> Result<(), ParserErr> {
    let parser = Parser::parse("model-xsleza26.txt")?;

    let objects: Vec<&Object> = parser.object.iter().collect();
    let attrs: Vec<&Attr> = parser.attr.iter().collect();
    /*
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
    */
    id3(&objects, &attrs);

    Ok(())
}

fn id3(mp: &Vec<&Object>, mv: &Vec<&Attr>) {
    let cnt = class_cnt(mp);
    if cnt.len() == 1 {
        // Return leaf node with the one class
        println!("uzel [label=\"{}\"]", mp.first().unwrap().class);
        return;
    }

    if mv.is_empty() {
        // Return leaf node with disjunction of all classes in mp
        println!("uzel disjunkce");
        return;
    }

    // Make attribute with highest entropy node, remove it from mv and continue
    let emp = cnt.values().map(|v| entropy(*v, mp.len())).sum();
    let mut e: Vec<f64> = vec![];
    let mut label = Vec::<String>::new();
    for (id, attr) in mv.iter().enumerate() {
        let val = attr_entropy(emp, id, &attr.values, mp);
        e.push(val);
        label.push(format!("{}={:.4}", attr.name, val));
    }

    let max = e
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(id, _)| id)
        .unwrap_or(0);
    println!("ruzel [label=\"{}|{{{}}}\"]", mv[max].name, label.join("|"));

    let mut mv = mv.clone();
    let attr = mv.remove(max);

    // For each value of the attribute create new branch
    // (mp with only that value)
    for value in attr.values.iter() {
        let mpi: Vec<&Object> = mp
            .iter()
            .filter(|o| o.attr.contains(&value))
            .map(|v| *v)
            .collect();
        println!(
            "uzel -> uzel [label=\"{} {{{}}}\"]",
            value,
            mpi.iter()
                .map(|o| o.id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        id3(&mpi, &mv);
    }
}

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
