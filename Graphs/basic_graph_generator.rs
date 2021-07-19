/// This is a basic adjacency list generator, that takes edges as input of form (src, dest, cost)
/// This is not an API

use std::io;

struct Edge {
    src: i32,
    dest: i32,
    cost: i32,
}

struct Vertex {
    label: i32,
    cost: i32,
}

/* Adj List func for directed graph */
fn di_graph(vector_edges: &Vec<Edge>) {
    let mut adj_list: Vec<Vec<Vertex>> = Vec::new();

    let mut size = i32::MIN;
    for e in vector_edges {
        size = std::cmp::max(size, std::cmp::max(e.dest, e.src));
    } 

    size += 1;

    let track: i32 = 0;
    while &size > &track {
        let vec_here: Vec<Vertex> = Vec::new();
        adj_list.push(vec_here);
        size -= 1;
    }

    for e in vector_edges {
        let node = Vertex {
            label: e.dest,
            cost: e.cost,
        };
        
        let src_idx = e.src as usize;
        adj_list[src_idx].push(node);
    }

    let mut vec_size = adj_list.len();
    let mut idx: usize = 0;

    // Printing the graph
    while vec_size > 0 {
        for v in &adj_list[idx] {
            print!("{:?} -> {:?}({:?}) ", idx, v.label, v.cost);
        }
        idx += 1;
        vec_size -= 1;
        println!();
    }
}

fn graph(vector_edges: &Vec<Edge>) {
    let mut adj_list: Vec<Vec<Vertex>> = Vec::new();

    let mut size = i32::MIN;

    for e in vector_edges {
        size = std::cmp::max(size, std::cmp::max(e.dest, e.src));
    }

    size += 1;

    let track = 0;
    while &size > &track {
        let vec_here: Vec<Vertex> = Vec::new();
        adj_list.push(vec_here);
        size -= 1;
    }

    for e in vector_edges {
        let src_idx = e.src as usize;
        let dest_idx = e.dest as usize;

        let node = Vertex {
            label: e.dest,
            cost: e.cost,
        };
        adj_list[src_idx].push(node);

        let node = Vertex {
            label: e.src,
            cost: e.cost,
        };
        adj_list[dest_idx].push(node);
    }

    let mut adj_list_size = adj_list.len();
    let mut idx: usize = 0;

    while adj_list_size > 0 {
        for v in &adj_list[idx] {
            println!("{:?} <-> {:?}({:?})", idx, v.label, v.cost);
        }
        idx += 1;
        adj_list_size -= 1;
        println!();
    }
}

fn main() {
    // Not to be added in lib.rs 
    interactive_ip();
}

fn interactive_ip() {
    println!("Interative Graph API");
    println!("Enter the number of edges: ");

    let mut n: i32 = config_ip();

    let mut vector_edges: Vec<Edge> = Vec::new();

    println!("Enter details about edges <src, dest, cost>");
    while n > 0 {
        let edge_data: Vec<i32> = get_edge_data();

        let edge_here = Edge {
            src: edge_data[0],
            dest: edge_data[1],
            cost: edge_data[2],
        };

        vector_edges.push(edge_here);
        n -= 1;
    }

    graph(&vector_edges);
    di_graph(&vector_edges);
}

fn config_ip() -> i32 {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Invalid");
    let ret = ip.trim().parse().expect("Invalid");
    ret
}

fn get_edge_data() -> Vec<i32> {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Invalid");
    let ret: Vec<i32> = ip.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
    ret
}



/* graph
Test
0 1 5
1 2 6
2 3 7
3 0 8
*/
