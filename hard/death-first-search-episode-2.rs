use std::collections::{HashSet, VecDeque};
use std::io;
use std::borrow::BorrowMut;
use std::collections::hash_set::Iter;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], i32); // the total number of nodes in the level, including the gateways
    let l = parse_input!(inputs[1], i32); // the number of links
    let e = parse_input!(inputs[2], i32); // the number of exit gateways

    let mut graph: Vec<HashSet<i32>> = (0..n).into_iter()
        .map(|_| -> HashSet<i32> {HashSet::new()})
        .collect();
    // let mut vertices:Vec<HashSet<i32>> = Vec::with_capacity(n as usize);
    let mut exit_list: Vec<i32> = Vec::new();


    for i in 0..l as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n1 = parse_input!(inputs[0], i32); // N1 and N2 defines a link between these nodes
        let n2 = parse_input!(inputs[1], i32);
        // let n1u: usize = n1;
        graph[n1 as usize].insert(n2);
        graph[n2 as usize].insert(n1);
    }
    for i in 0..e as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let ei = parse_input!(input_line, i32); // the index of a gateway node
        exit_list.push(ei);
    }

    // println!("{:?}", graph);
    // return ();

    let mut network = Network {
        graph: graph,
        exits: exit_list.clone(),
    };

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let si = parse_input!(input_line, i32); // The index of the node on which the Skynet agent is positioned this turn

        let edge = get_best_exit(&network, &exit_list, si);

        println!("{} {}", edge.0, edge.1);

        network.remove_edge(edge);
    }
}

struct Node {
    index: i32,
    distance_from_virus: i32,
    adjacent_exits_number: i32,
    priority: i32,
}

impl Node {
    fn new(index: i32, network: &Network, paths: &Vec<i32>, si: i32) -> Node {
        let distance_from_virus = get_path_length(&paths, index, si);
        let adjacent_exits_number = network.get_red_edges_count_for_node(index);

        let path = Paths::get_path_for_node_2(paths, index);
        let red_edges_count = network.get_red_edges_count_for_path(&path);

        Node {
            index,
            distance_from_virus,
            adjacent_exits_number,
            // priority: distance_from_virus - adjacent_exits_number - 1,
            priority: red_edges_count - path.len() as i32,
        }
    }
}

fn get_best_exit(network: &Network, exits: &Vec<i32>, si: i32) -> (i32, i32) {
    let (_paths, mut nodes) = create_paths_and_nodes(&network, &exits, si);

    nodes.sort_by(|a, b| {
        if a.priority != b.priority {
            return b.priority.cmp(&a.priority);
        }

        // if a.adjacent_exits_number != b.adjacent_exits_number {
        //     return b.adjacent_exits_number.cmp(&a.adjacent_exits_number);
        // }

        return a.distance_from_virus.cmp(&b.distance_from_virus);
    });

    let most_prioritized_hub = nodes[0].index;
    let exit: i32 = network.iter_neighbors(most_prioritized_hub)
        .find(|n| exits.contains(n))
        .unwrap().clone();

    (most_prioritized_hub, exit)
}

fn create_paths_and_nodes(network: &Network, exits: &Vec<i32>, si: i32) -> (Paths, Vec<Node>) {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut paths: Vec<i32> = vec![-1; network.graph.len()];
    let mut queue: VecDeque<i32> = VecDeque::new();

    let mut nodes: Vec<Node> = Vec::new();

    queue.push_back(si);

    while !queue.is_empty() {

        let current = queue.pop_front().unwrap();
        visited.insert(current);
        if !exits.contains(&current) {
            let node = Node::new(current, &network, &paths, si);
            if node.adjacent_exits_number > 0 {
                nodes.push(node);
            }
        }


        for &neighbor in network.iter_neighbors(current) {
            if exits.contains(&neighbor) {
                continue;
            }

            if !visited.contains(&neighbor) {
                paths[neighbor as usize] = current;
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    (Paths {parents: paths}, nodes)
}

fn get_path_length(paths: &Vec<i32>, exit: i32, si: i32) -> i32 {
    let mut length = 0;
    let mut current = exit;
    loop {
        let parent = paths[current as usize];

        if parent == -1 {
            return 0;
        }

        current = parent;
        length += 1;

        if parent == si {
            break;
        }
    }

    length
}

struct Network {
    graph: Vec<HashSet<i32>>,
    exits: Vec<i32>,
}

impl Network {
    fn remove_edge(&mut self, edge: (i32, i32)) {
        self.graph[edge.0 as usize].remove(&edge.1);
        self.graph[edge.1 as usize].remove(&edge.0);
    }

    fn get_red_edges_count_for_path(&self, path: &Vec<i32>) -> i32 {
        let mut count = 0;

        for &node in path {
            count += self.get_red_edges_count_for_node(node);
        }

        count
    }

    fn get_red_edges_count_for_node(&self, node: i32) ->i32 {
        // self.graph[node as usize].iter().filter(|n| {
        //     self.exits.contains(n)
        // }).count() as i32
        self.iter_neighbors(node)
            .filter(|n| {
                self.exits.contains(n)
            })
            .count() as i32
    }

    fn iter_neighbors(&self, node: i32) -> Iter<i32> {
        self.graph[node as usize].iter()
    }
}

struct Paths {
    parents: Vec<i32>,
}

impl Paths {
    fn get_path_for_node(&self, node: i32) -> Vec<i32> {
        let mut path: Vec<i32> = Vec::new();
        let mut current = node;
        path.push(node);

        loop {
            let parent = self.parents[current as usize];
            if parent == -1 {
                break;
            }
            path.push(parent);
            current = parent;
        }

        path
    }

    fn get_path_for_node_2(parents: &Vec<i32>, node: i32) -> Vec<i32> {
        let mut path: Vec<i32> = Vec::new();
        let mut current = node;
        path.push(node);

        loop {
            let parent = parents[current as usize];
            if parent == -1 {
                break;
            }
            path.push(parent);
            current = parent;
        }

        path
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod parent_to_path {
        use super::*;

        #[test]
        fn test_single_node() {
            let paths = Paths {
                parents: vec![-1, 0, 1]
            };

            let path = paths.get_path_for_node(0);

            assert_eq!(path, vec![0]);
        }

        #[test]
        fn test_three_nodes() {
            let paths = Paths {
                parents: vec![-1, 0, 1]
            };

            let path = paths.get_path_for_node(2);

            assert_eq!(path, vec![2, 1, 0]);
        }
    }
}
