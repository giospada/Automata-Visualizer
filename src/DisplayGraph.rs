use crate::Log::*;
use egui::{emath::RectTransform, Color32, Pos2, Vec2};

pub struct DisplayGraph {
    edges: Vec<(usize, usize, Option<char>)>,
    nodes: Vec<String>,
    nodes_pos: Vec<Pos2>,
    nodes_bfs: Vec<Vec<usize>>,
    last_parameter: DisplayGraphParameter,
}

#[derive(PartialEq, Copy, Clone)]
pub struct DisplayGraphParameter {
    pub padding_x: f32,
    pub padding_y: f32,
    pub node_size: f32,
}

impl DisplayGraphParameter {
    pub fn invalid() -> Self {
        DisplayGraphParameter {
            padding_x: -1.,
            padding_y: -1.,
            node_size: -1.,
        }
    }
}

impl DisplayGraph {
    pub fn new(
        edges: Vec<(usize, usize, Option<char>)>,
        nodes: Vec<String>,
        nodes_bfs: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            edges: edges,
            nodes_pos: vec![Pos2 { x: 0., y: 1. }; nodes.len()],
            nodes: nodes,
            nodes_bfs: nodes_bfs,
            last_parameter: DisplayGraphParameter::invalid(),
        }
    }

    pub fn position(&mut self, params: DisplayGraphParameter) -> Vec2 {
        let width = self
            .nodes_bfs
            .iter()
            .map(|nodes| nodes.len())
            .max()
            .unwrap_or(0) as f32;
        let depth = self.nodes_bfs.len();
        if params != self.last_parameter {
            log!("recalculated ");
            let mx = width as f32 * (params.node_size + params.padding_x) + params.padding_x;
            for (c_depth, nodes_level) in self.nodes_bfs.iter().enumerate() {
                for (index, node) in nodes_level.iter().enumerate() {
                    self.nodes_pos[*node] = Pos2 {
                        x: (index as f32 + 1.) * (mx / (width as f32 + 1.)),
                        y: (c_depth as f32) * (params.node_size + params.padding_y)
                            + params.padding_y,
                    };
                }
            }
            self.last_parameter = params;
        }
        Vec2 {
            x: width as f32 * (params.node_size + params.padding_x) + params.padding_x,
            y: depth as f32 * (params.node_size + params.padding_y) + params.padding_y,
        }
    }

    pub fn draw(
        &self,
        painter: &egui::Painter,
        to_screen: RectTransform,
        ui: &egui::Ui,
        _response: &mut egui::Response,
    ) {
        //display edges
        for (from, to, label) in &self.edges {
            let orgin=to_screen.transform_pos(self.nodes_pos[*from]);
            let end=to_screen.transform_pos(self.nodes_pos[*to]);
            let vec= (end-orgin.to_vec2()).to_vec2();
            let dist=Pos2{x:self.last_parameter.node_size/2.,y:self.last_parameter.node_size/2.};
            let direction=vec.normalized()*dist.to_vec2();
            
            painter.arrow(
                orgin,
                vec-direction,
                egui::Stroke::new(3 as f32, egui::Color32::BLUE),
            );

            if let Some(label) = label {
                let pos = self.nodes_pos[*from]
                    + (self.nodes_pos[*to] - self.nodes_pos[*from].to_vec2()).to_vec2() / 2.;
                let pos = to_screen.transform_pos(pos);
                painter.text(
                    pos,
                    egui::Align2::CENTER_CENTER,
                    label.to_string(),
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::GRAY,
                );
            }
        }
        //display nodes

        for (index, node) in self.nodes.iter().enumerate() {
            let pos = to_screen.transform_pos(self.nodes_pos[index]);

            painter.circle_filled(
                pos,
                self.last_parameter.node_size / 2.0,
                egui::Color32::WHITE,
            );
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                node.clone(),
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::BLACK,
            );
        }
    }
}

pub trait ToDisplayGraph {
    fn to_display_graph(&self) -> DisplayGraph;
}
