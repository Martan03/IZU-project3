use std::collections::HashMap;

use crate::{
    node::Node,
    object::{Attr, Object},
};

/// Generates desicion tree using id3 algorithm
pub fn id3(id: &mut usize, mp: &Vec<&Object>, mv: &Vec<&Attr>) -> Node {
    let cnt = class_cnt(mp);
    if cnt.len() == 1 {
        return class_leaf(id, mp);
    }

    if mv.is_empty() {
        return dis_class_leaf(id, mp);
    }

    // Make attribute with highest entropy node, remove it from mv and continue
    let emp = cnt.values().map(|v| entropy(*v, mp.len())).sum();
    let mut e: Vec<f64> = vec![];
    let mut label = Vec::<String>::new();
    for attr in mv.iter() {
        let val = attr_entropy(emp, attr.id, &attr.values, mp);
        e.push(val);
        label.push(format!("{}={:.4}", attr.name, val));
    }

    let max = e
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(id, _)| id)
        .unwrap_or(0);

    // For each value of the attribute create new branch
    // (mp with only that value)
    let mut node = Node::leaf(
        format!("uzel{}", *id),
        format!("{}|{{{}}}", mv[max].name, label.join("|")),
    );

    let mut mv = mv.clone();
    let attr = mv.remove(max);
    *id += 1;

    for value in attr.values.iter() {
        let mpi: Vec<&Object> = mp
            .iter()
            .filter(|o| o.attr.contains(&value))
            .map(|v| *v)
            .collect();
        if mpi.is_empty() {
            continue;
        }
        let mut child = id3(id, &mpi, &mv);
        child.trans = format!(
            "{} {{{}}}",
            value,
            mpi.iter()
                .map(|o| o.id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        node.children.push(child);
    }
    node
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
        let val = (obj.len() as f64 / objs.len() as f64)
            * cnt.values().map(|v| entropy(*v, obj.len())).sum::<f64>();
        e -= val;
    }
    e
}

fn entropy(dividend: usize, divisor: usize) -> f64 {
    let frac = dividend as f64 / divisor as f64;
    -frac * f64::log2(frac)
}

/// Gets class leaf
fn class_leaf(id: &mut usize, mp: &Vec<&Object>) -> Node {
    *id += 1;
    Node::leaf(
        format!("uzel{}", *id - 1),
        mp.first().unwrap().class.to_string(),
    )
}

/// Gets class leaf with class disjunction
fn dis_class_leaf(id: &mut usize, mp: &Vec<&Object>) -> Node {
    *id += 1;
    Node::leaf(
        format!("uzel{}", *id - 1),
        mp.iter()
            .map(|o| o.class.to_string())
            .collect::<Vec<String>>()
            .join("^"),
    )
}
