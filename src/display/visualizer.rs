use crate::display::DisplayGraph;


/// this struct rappresent a visualizer of a graph
/// it contains the information to show the window and display the graph
pub struct Visualizer {
    pub box_title: String,
    pub graph: Option<DisplayGraph>,
    pub size_node: f32,
    pub padding_y: f32,
    pub padding_x: f32,
    pub is_win_open: bool,

    // we can add a lot of paramters such color of nodes, etc..
}

impl Visualizer {
    pub fn new(box_title: String) -> Self {
        Self {
            box_title: box_title,
            graph: None,
            is_win_open: false,
            padding_x: 40.,
            padding_y: 40.,
            size_node: 30.,
        }
    }

    pub fn check_open(&mut self) {
        if let None = self.graph {
            self.is_win_open = false;
        }
        if !self.is_win_open {
            self.graph = None;
        }
    }

    pub fn set_graph(&mut self, graph: DisplayGraph) {
        self.graph = Some(graph);
        self.is_win_open = true;
    }
}
