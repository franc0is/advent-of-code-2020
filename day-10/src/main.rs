use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use itertools::Itertools;
use gamma::graph::{ Graph, DefaultGraph };
use std::collections::{HashMap};

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let adapters = buf.lines().map(|l| l.unwrap().parse().unwrap()).sorted();

    // part 1
    let mut last = 0;
    let mut sums = [0, 0, 0];
    for adapter in adapters.clone() {
        sums[(adapter - last - 1) as usize] += 1;
        last = adapter;
    }
    // add device (always 3 away)
    sums[2] += 1;
    println!("part 1 {}", sums[0] * sums[2]);

    // part 2
    let mut graph = DefaultGraph::new();
    let mut trailing: Vec<usize> = [0].to_vec();
    graph.add_node(0).unwrap();
    for adapter in adapters.clone() {
        graph.add_node(adapter).unwrap();
        for node in &trailing {
            if adapter - node <= 3 {
                graph.add_edge(adapter, *node).unwrap();
            }
        }
        trailing.push(adapter);
        if trailing.len() > 3 {
            trailing.remove(0);
        }
    }
    let last = adapters.last().unwrap();
    graph.add_node(last + 3).unwrap();
    graph.add_edge(last, last + 3).unwrap();

    fn get_children(graph: &DefaultGraph, node: &usize) -> Vec<usize> {
        let children: Vec<usize> = graph.neighbors(*node).unwrap().iter().filter(|n| *n > node).map(|n| *n).collect();
        return children
    }

    fn count_paths(graph: &DefaultGraph, paths_cache: &mut HashMap<usize, u64>, start: usize, end: usize) -> u64 {
        if start >= end {
            return 1;
        }

        if !paths_cache.contains_key(&start) {
            let children = get_children(graph, &start);
            let p = children.iter().fold(0, |acc, child| acc + count_paths(graph, paths_cache, *child, end));
            paths_cache.insert(start, p);
        }

        return *paths_cache.get(&start).unwrap();
    }

    let mut paths_cache: HashMap<usize, u64> = HashMap::new();
    println!("Part 2: {}", count_paths(&graph, &mut paths_cache, 0, last + 3));
}
