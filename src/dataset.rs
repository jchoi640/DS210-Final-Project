use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::{HashMap};

pub fn read_file(path: &str) -> Result<(usize, Vec<(usize, usize)>), Error> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    let mut vertices = 0;

    for line in buf_reader.lines() {
        let line_str = line?;
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        if v.len() == 1 {
            vertices = v[0].parse().expect("Failed to parse vertex count");
        } else {
            let label = v[0].parse().expect("Failed to parse label");
            let edge = v[1].parse().expect("Failed to parse edge");
            contents.push((label, edge));
        }
    }
    Ok((vertices, contents))
}

pub fn create_adjacency_list(edges: &[(usize,usize)]) -> HashMap<usize, Vec<usize>> {
    let mut adjacency_list = HashMap::new();

    for &(vertex1, vertex2) in edges {
        adjacency_list
            .entry(vertex1)
            .or_insert(vec![])
            .push(vertex2);

        adjacency_list
            .entry(vertex2)
            .or_insert(vec![])
            .push(vertex1);
    }
    adjacency_list
}