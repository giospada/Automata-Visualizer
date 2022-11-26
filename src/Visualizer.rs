use crate::DisplayGraph::*;

pub struct Visualizer {
    pub name: String,
    pub graph: Option<DisplayGraph>,
    pub size_node: f32,
    pub padding_y: f32,
    pub padding_x: f32,
    pub open: bool,
    // we can add a lot of paramters such color of nodes, etc..
}

pub trait VisualizerName {
    fn get_name() -> String;
}

impl Visualizer {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            graph: None,
            open: false,
            padding_x: 40.,
            padding_y: 40.,
            size_node: 30.,
        }
    }

    pub fn check_open(&mut self) {
        if let None = self.graph {
            self.open = false;
        }
        if !self.open {
            self.graph = None;
        }
    }

    fn set_graph(&mut self, graph: DisplayGraph) {
        self.graph = Some(graph);
        self.open = true;
    }
}

impl<T: Into<DisplayGraph> + VisualizerName> From<T> for Visualizer {
    fn from(graph: T) -> Self {
        let mut vis = Visualizer::new(<T as VisualizerName>::get_name());
        let graph: DisplayGraph = graph.into();
        vis.set_graph(graph);
        vis
    }
}