
use std::collections::{BTreeSet,VecDeque};

type IndNode = usize;
type IndEdge = usize;

//TODO label can be set to a generic type
pub struct Edge {
    id:IndEdge,
    from:IndNode ,
    to:IndNode,
    label:Option<String>
}


//TODO label can be set to a generic type
pub struct Node{
    id:IndNode,
    label:Option<String>,
    edges:Vec<IndEdge>,
}


// in this implementation the graph can be only added edge and node
// the remove of both will be implmentent in a next version
pub struct Graph{
    nodes:Vec<Node>,
    edges:Vec<Edge>
}

impl Graph {
    pub fn new()->Self{
        Self{
            nodes:vec![],
            edges:vec![]
        }
    }
    pub fn addNode(&mut self,label:Option<String>)->IndNode{
        let id=self.nodes.len();
        let mut node=Node{
            id,
            label,
            edges:vec![]
        };
        self.nodes.push(node);
        id
    }
    pub fn addEdge(&mut self,from:IndNode,to:IndNode,label:Option<String>)->IndEdge{
        let id=self.nodes.len();
        //TODO in future version we need to check if the indNodee are inside the graph 
        let edge=Edge{
            id,
            from,
            to,
            label
        };
        self.nodes[from].edges.push(id);
        id
    }
    pub fn bfs(&self,start_node:IndNode)->Vec<Vec<IndNode>>{
        //store all index
        let mut set:BTreeSet<IndNode>=(0..self.nodes.len()-1).into_iter().collect();
        let mut output=self.bfs_connected_graph(start_node,&mut set);
        while set.is_empty(){
            match set.iter().next() {
                None => break,
                Some(val) => {
                    let mut temp=self.bfs_connected_graph(*val,&mut set);
                    output.append(&mut temp);
                }
            }
        }
        output
    }

    fn bfs_connected_graph(&self,start_node:IndNode,not_explored:&mut BTreeSet<IndNode>)->Vec<Vec<IndNode>>{
        let mut queue=VecDeque::from([Some(start_node),None]);
        let mut output=vec![];
        let mut current_level=vec![];
        while let Some(current) = queue.pop_front() {
            match current{
                None => {
                   output.push(current_level);
                   current_level=vec![];
                    if queue.is_empty(){
                        break;
                    }else{
                        queue.push_back(None);
                    }
                },
                Some(ind) => {
                    self.nodes[ind].edges.iter().for_each(|edge_ind|{
                        let to=self.edges[*edge_ind].to;
                        if not_explored.contains(&to) {
                            not_explored.remove(&to);
                            queue.push_back(Some(to))
                        }
                    });
                    current_level.push(ind);
                },
            }
        }
        output
    }
}


#[cfg(test)]
mod test {
    use super::*;

    // TODO graph cration
    #[test]
    fn bfs_test(){
        let mut g=Graph::new();
        let s=g.addNode(None);
        let other:Vec<IndNode>=(0..4).into_iter().map(|_| {g.addNode(None)}).collect();
        g.addEdge(s,other[0],None);
        g.addEdge(s,other[1],None);
        g.addEdge(other[0],other[2],None);

        assert_eq!(vec![vec![0],vec![1,2],vec![3],vec![4]],g.bfs(s));
        
    }

}
