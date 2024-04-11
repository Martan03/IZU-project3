use std::collections::HashMap;

use crate::{
    node::Node,
    object::{Attr, Object},
};

/// Generates desicion tree using id3 algorithm
pub fn id3(id: &mut usize, mp: &Vec<&Object>, mv: &[&Attr]) -> Node {
    let cnt = class_cnt(mp);
    if cnt.len() == 1 {
        return class_leaf(id, mp);
    }

    if mv.is_empty() {
        return dis_class_leaf(id, mp);
    }

    let (max, labels) = highest_gain(&cnt, mp, mv);
    let mut node = Node::leaf(
        format!("uzel{}", *id),
        format!("{}|{{{}}}", mv[max].name, labels.join("|")),
    );

    let mut mv = mv.to_owned();
    let attr = mv.remove(max);
    *id += 1;

    for value in attr.values.iter() {
        let mpi: Vec<&Object> = mp
            .iter()
            .filter(|o| o.attr[attr.id] == *value)
            .copied()
            .collect();
        if mpi.is_empty() {
            continue;
        }
        node.add_child(id3(id, &mpi, &mv), value, &mpi);
    }
    node
}

/// Gets count of each class
fn class_cnt(objects: &Vec<&Object>) -> HashMap<String, usize> {
    let mut cnt: HashMap<String, usize> = HashMap::new();
    for obj in objects {
        cnt.entry(obj.class.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    cnt
}

/// Gets index of the attribute with highest gain and labels
fn highest_gain(
    cnt: &HashMap<String, usize>,
    mp: &[&Object],
    mv: &[&Attr],
) -> (usize, Vec<String>) {
    let emp = cnt.values().map(|v| entropy(*v, mp.len())).sum();
    let mut labels = Vec::<String>::new();

    let mut max = 0.0;
    let mut max_id = 0;
    for (id, attr) in mv.iter().enumerate() {
        let val = attr_gain(emp, attr.id, &attr.values, mp);
        labels.push(format!("{}={:.4}", attr.name, val));

        if val > max {
            max = val;
            max_id = id;
        }
    }
    (max_id, labels)
}

/// Gets attribute gain
fn attr_gain(
    emp: f64,
    id: usize,
    vals: &Vec<String>,
    objs: &[&Object],
) -> f64 {
    let mut e = emp;
    for v in vals {
        let obj = objs.iter().filter(|x| x.attr[id] == *v).copied().collect();

        let cnt = class_cnt(&obj);
        let val = (obj.len() as f64 / objs.len() as f64)
            * cnt.values().map(|v| entropy(*v, obj.len())).sum::<f64>();
        e -= val;
    }
    e
}

/// Gets entropy
fn entropy(dividend: usize, divisor: usize) -> f64 {
    let frac = dividend as f64 / divisor as f64;
    -frac * f64::log2(frac)
}

/// Gets class leaf
fn class_leaf(id: &mut usize, mp: &[&Object]) -> Node {
    *id += 1;
    Node::leaf(
        format!("uzel{}", *id - 1),
        mp.first().unwrap().class.to_string(),
    )
}

/// Gets class leaf with class disjunction
fn dis_class_leaf(id: &mut usize, mp: &[&Object]) -> Node {
    *id += 1;
    Node::leaf(
        format!("uzel{}", *id - 1),
        mp.iter()
            .map(|o| o.class.to_string())
            .collect::<Vec<String>>()
            .join("^"),
    )
}
