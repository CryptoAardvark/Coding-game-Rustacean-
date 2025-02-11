use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

#[derive(Default, Clone)]
struct Node {
    links: HashSet<usize>,
    gateway: bool,
    links2gw: HashSet<usize>,
}

fn bfs(graph: &mut Vec<Node>, s: usize) -> (usize, usize) {
    let mut visited = vec![false; graph.len()];
    let mut q = VecDeque::new();
    q.push_back(s);
    visited[s] = true;

    let mut selected_node = None;

    while let Some(id) = q.pop_front() {
        let n = &graph[id];
        visited[id] = true;

        let mut push_neighbours = || {
            for &nid in &n.links {
                if !visited[nid] {
                    q.push_back(nid);
                }
            }
        };

        if n.links2gw.len() > 1 {
            selected_node = Some(id);
            break;
        } else if n.links2gw.len() == 1 {
            if selected_node.is_none() {
                selected_node = Some(id);
                eprintln!(" selectedNode: {}", id);
                if id == s {
                    break;
                }
            }
            push_neighbours();
        } else if selected_node.is_none() {
            push_neighbours();
        }
    }

    let selected_node = selected_node.unwrap();
    (selected_node, *graph[selected_node].links2gw.iter().next().unwrap())
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let l: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    let mut graph = vec![Node::default(); n];

    for _ in 0..l {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n1: usize = parts.next().unwrap().parse().unwrap();
        let n2: usize = parts.next().unwrap().parse().unwrap();
        graph[n1].links.insert(n2);
        graph[n2].links.insert(n1);
    }

    let mut gateways = Vec::new();
    for _ in 0..e {
        let line = lines.next().unwrap().unwrap();
        let ei: usize = line.parse().unwrap();
        graph[ei].gateway = true;
        gateways.push(ei);
    }

    for &ei in &gateways {
        let links: Vec<usize> = graph[ei].links.iter().cloned().collect();
        for nid in links {
            graph[nid].links2gw.insert(ei);
        }
    }

    loop {
        let line = lines.next().unwrap().unwrap();
        let si: usize = line.parse().unwrap();

        let (node, gateway) = bfs(&mut graph, si);
        println!("{} {}", node, gateway);

        // Cut link:
        graph[node].links2gw.remove(&gateway);
        graph[node].links.remove(&gateway);
        graph[gateway].links.remove(&node);
    }
}