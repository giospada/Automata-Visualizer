use std::collections::{BTreeSet, VecDeque};

// we youse this custom type because we want to access only
// the index returned by this struct, in a future they could be
// RefCell or somthing similar
pub type IndNode = usize;
pub type IndEdge = usize;

// maybe label can be set to a generic type
pub struct Edge {
    pub id: IndEdge,
    pub from: IndNode,
    pub to: IndNode,
    pub label: Option<String>,
}

// maybelabel can be set to a generic type
pub struct Node {
    pub id: IndNode,
    pub label: Option<String>,
    edges: Vec<IndEdge>,
}

// in this implementation the graph can be only added edge and node
// the remove of both will be implmentent in a next version
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    pub start_node: Option<IndNode>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            start_node: None,
        }
    }

    pub fn get_edges_ids(&self) -> Vec<IndEdge> {
        self.edges.iter().map(|edge| edge.id).collect()
    }

    pub fn get_nodes_ids(&self) -> Vec<IndNode> {
        self.nodes.iter().map(|node| node.id).collect()
    }
    pub fn get_node_label(&self, node_id: IndNode) -> &Option<String> {
        &self.nodes[node_id].label
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
    pub fn get_node_edges(&self, node_id: IndNode) -> &Vec<IndEdge> {
        &self.nodes[node_id].edges
    }

    pub fn add_node(&mut self, label: Option<String>) -> IndNode {
        let id = self.nodes.len();
        let node = Node {
            id,
            label,
            edges: vec![],
        };
        self.nodes.push(node);
        id
    }
    pub fn add_edge(&mut self, from: IndNode, to: IndNode, label: Option<String>) -> IndEdge {
        let id = self.edges.len();
        //TODO in future version we need to check if the indNodee are inside the graph
        let edge = Edge {
            id,
            from,
            to,
            label,
        };
        self.nodes[from].edges.push(id);
        self.edges.push(edge);
        id
    }

    pub fn bfs(&self, mut start_node: Option<IndNode>) -> Vec<Vec<IndNode>> {
        //store all index
        let mut set: BTreeSet<IndNode> = (0..self.nodes.len()).into_iter().collect();
        let mut bfs_exploring_order = vec![];

        // if the start node is not set we take the first node
        if start_node.is_none() {
            start_node = self.start_node;
        }

        while !set.is_empty() {
            // if start_node is not set we pick a random one
            if start_node.is_none() {
                start_node = set.iter().next().cloned();
            }
            match start_node {
                None => break,
                Some(val) => {
                    // we add an other connected component to the graph traversal
                    bfs_exploring_order.append(&mut self.bfs_connected_graph(val, &mut set));
                }
            }
            start_node = None;
        }
        bfs_exploring_order
    }

    fn bfs_connected_graph(
        &self,
        start_node: IndNode,
        not_explored: &mut BTreeSet<IndNode>,
    ) -> Vec<Vec<IndNode>> {
        // contains the nodes that we need to explore
        let mut queue = VecDeque::from([Some(start_node), None]);
        not_explored.remove(&start_node);
        let mut bfs_visit_order = vec![];

        let mut node_in_the_same_deepth = vec![];
        while let Some(current) = queue.pop_front() {
            match current {
                // indicates that is the end of the current level
                None => {
                    bfs_visit_order.push(node_in_the_same_deepth);
                    node_in_the_same_deepth = vec![];
                    if queue.is_empty() {
                        break;
                    } else {
                        queue.push_back(None);
                    }
                }
                Some(ind) => {
                    let node = &self.nodes[ind];
                    node.edges.iter().for_each(|edge_ind| {
                        let to = self.edges[*edge_ind].to;
                        if not_explored.contains(&to) {
                            not_explored.remove(&to);
                            queue.push_back(Some(to))
                        }
                    });
                    node_in_the_same_deepth.push(ind);
                }
            }
        }
        bfs_visit_order
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO graph cration
    #[test]
    fn bfs_test() {
        let mut g = Graph::new();
        let s = g.add_node(None);
        let other: Vec<IndNode> = (0..4).into_iter().map(|_| g.add_node(None)).collect();
        g.add_edge(s, other[0], None);
        g.add_edge(s, other[1], None);
        g.add_edge(other[0], other[2], None);

        assert_eq!(vec![vec![0], vec![1, 2], vec![3], vec![4]], g.bfs(Some(s)));
    }
}
