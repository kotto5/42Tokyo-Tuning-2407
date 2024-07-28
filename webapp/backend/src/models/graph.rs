use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

// Priority queue item
#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

// The priority queue depends on `Ord`.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In Rust, BinaryHeap is a max-heap by default.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();

        // Initialize distance to start node as 0 and push it to the heap
        distances.insert(from_node_id, 0);
        heap.push(State {
            cost: 0,
            position: from_node_id,
        });

        // Dijkstra's algorithm
        while let Some(State { cost, position }) = heap.pop() {
            // If we've reached the destination, return the cost
            if position == to_node_id {
                return cost;
            }

            // If the cost is greater than the recorded cost, continue
            if let Some(&current_cost) = distances.get(&position) {
                if cost > current_cost {
                    continue;
                }
            }

            // For each edge connected to the current position
            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next = State {
                        cost: cost + edge.weight,
                        position: edge.node_b_id,
                    };

                    // If this path is cheaper, record it and push it to the heap
                    let is_shorter = distances
                        .get(&next.position)
                        .map_or(true, |&current_cost| next.cost < current_cost);

                    if is_shorter {
                        distances.insert(next.position, next.cost);
                        heap.push(next);
                    }
                }
            }
        }

        // If the destination is unreachable, return a large number
        i32::MAX
    }
}
