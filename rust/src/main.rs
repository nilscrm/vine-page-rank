use rand::prelude::*;
use rand_pcg::Pcg32;

struct Node {
    outgoing_edges: Vec<usize>,
    page_rank: f32,
}

impl Node {
    fn new(total_num_nodes: u32) -> Self {
        Node {
            outgoing_edges: Vec::new(),
            page_rank: 1.0 / total_num_nodes as f32,
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(num_nodes: u32) -> Self {
        Graph {
            nodes: (0..num_nodes).map(|_| Node::new(num_nodes)).collect(),
        }
    }

    fn connect(&mut self, a: u32, b: u32) {
        if a != b {
            let a = a as usize;
            let b = b as usize;
            self.nodes[a].outgoing_edges.push(b);
        }
    }

    fn process(&mut self, damping: f32) {
        let num_nodes = self.nodes.len();
        let mut contributions = vec![0.0; num_nodes];

        // Calculate contributions from each node
        for node in self.nodes.iter() {
            let num_outgoing = node.outgoing_edges.len();
            if num_outgoing > 0 {
                let contribution = damping * node.page_rank / num_outgoing as f32;
                for &target in &node.outgoing_edges {
                    contributions[target] += contribution;
                }
            }
        }

        // Update page ranks
        for (i, node) in self.nodes.iter_mut().enumerate() {
            let base_rank = (1.0 - damping) / num_nodes as f32;
            node.page_rank = base_rank + contributions[i];
        }
    }

    fn print(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            println!("Node {}: {}", i, node.page_rank);
        }
    }
}

fn main() {
    let num_nodes = 200;
    let num_edges = 4000;
    let iterations = 1000;
    let mut graph = Graph::new(num_nodes);

    let mut rng = Pcg32::seed_from_u64(12345);
    for _ in 0..num_edges {
        let a = rng.random_range(0..num_nodes);
        let b = rng.random_range(0..num_nodes);
        graph.connect(a, b);
    }

    let damping = 0.85;
    for _ in 0..iterations {
        graph.process(damping);
    }

    graph.print();
}
