use egui::{Color32, Pos2, Vec2,Stroke,emath};
use crate::Log::*;

pub struct SyntaxTree {
    pub label: String,
    pub pos: Pos2,
    pub size: f32,
    pub children: Vec<SyntaxTree>,
}

impl SyntaxTree {
    pub fn from_label(label: String) -> SyntaxTree {
        SyntaxTree {
            label,
            pos: Pos2::ZERO,
            size: 0.,
            children: Vec::new(),
        }
    }
    fn deepth(&self) -> usize {
        let mut max = 0;
        for child in &self.children {
            max = max.max(child.deepth());
        }
        max + 1
    }
    fn max_width(&self) -> usize {
        //bfs to caclulate the with of the tree
        let mut max = 0;
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_front(self);
        let mut current_size = 1;
        while let Some(node) = queue.pop_front() {
            current_size = current_size - 1;
            for child in &node.children {
                queue.push_back(child);
            }
            if current_size == 0 {
                max = max.max(queue.len());
                current_size = queue.len();
            }
        }
        max
    }

    pub fn draw_tree(
        &self,
        painter: &egui::Painter,
        to_screen: emath::RectTransform,
        ui: &egui::Ui,
        response: &mut egui::Response,
    ) {
        for child in &self.children {
            painter.line_segment([to_screen.transform_pos(self.pos),to_screen.transform_pos(child.pos)],Stroke::new(5 as f32,Color32::YELLOW) );
            child.draw_tree(painter, to_screen, ui, response);
        }

        log!("{:?}", to_screen.transform_pos(self.pos));
        painter.circle_filled(to_screen.transform_pos(self.pos), self.size / 2.0, egui::Color32::WHITE);
        painter.text(
            to_screen.transform_pos(self.pos),
            egui::Align2::CENTER_CENTER,
            self.label.clone(),
            egui::TextStyle::Body.resolve(ui.style()),
            Color32::BLACK,
        );
    }


    pub fn position_tree(&mut self, padding:Pos2,node_size:f32) -> Vec2 {
        let depth = self.deepth();
        let max_width = self.max_width();
        let mx= max_width as f32 *(node_size+padding.x)+padding.x;
        let my= depth as f32 *(node_size+padding.y)+padding.y;
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_front((self, 0));
        let mut current_size = 1;
        let mut current_node = 0;
        while let Some((node, depth)) = queue.pop_front() {
            for child in node.children.iter_mut() {
                queue.push_back((child, depth + 1));
            }
            node.size = node_size;
            node.pos = Pos2::new(
                ((current_node + 1 )as f32) * (mx / ((current_size +1) as f32)),
                (depth as f32)*(node_size+padding.y)+padding.y,
            );
            current_node = current_node + 1;
            if current_size == current_node {
                current_node = 0;
                current_size = queue.len();
            }
        }
        Vec2{ x:mx,y:my }
    }
}

pub trait ToSingleTree {
    fn to_syntax_tree(&self) -> SyntaxTree;
    fn label(&self) -> String;
}
