use eframe::emath::RectTransform;
use egui::{Color32, Pos2, Rect, Vec2,Stroke};

#[must_use]
use crate::Log::*;

pub struct SyntaxTree {
    pub label: String,
    pub pos: Pos2,
    pub size: Vec2,
    pub children: Vec<SyntaxTree>,
}

impl SyntaxTree {
    pub fn from_label(label: String) -> SyntaxTree {
        SyntaxTree {
            label,
            pos: Pos2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
        }
    }
    pub fn deepth(&self) -> usize {
        let mut max = 0;
        for child in &self.children {
            max = max.max(child.deepth());
        }
        max + 1
    }
    pub fn max_width(&self) -> usize {
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
        padding: Vec2,
        ui: &egui::Ui,
        response: &mut egui::Response,
    ) {
        for child in &self.children {
            painter.line_segment([self.pos+padding,child.pos+padding],Stroke::new((5 as f32) ,Color32::YELLOW) );
            child.draw_tree(painter, padding, ui, response);
        }

        painter.circle_filled(self.pos + padding, self.size.x / 2.0, egui::Color32::WHITE);
        painter.text(
            self.pos + padding,
            egui::Align2::CENTER_CENTER,
            self.label.clone(),
            egui::TextStyle::Body.resolve(ui.style()),
            Color32::BLACK,
        );
    }

    pub fn position_tree(&mut self, screen_size: Pos2) {
        let depth = self.deepth();
        let max_width = self.max_width();
        let row_height = screen_size.y / (depth * 2 + 2) as f32;
        let col_width = screen_size.x / (max_width * 2 + 2) as f32;
        let node_size = row_height.min(col_width);
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_front((self, 0));
        let mut current_size = 1;
        let mut current_node = 0;
        while let Some((node, depth)) = queue.pop_front() {
            for child in node.children.iter_mut() {
                queue.push_back((child, depth + 1));
            }
            node.size = Vec2::new(node_size, node_size);
            node.pos = Pos2::new(
                ((current_node as f32) * 2.0 + 1.0) * (screen_size.x / ((current_size * 2) as f32)),
                (depth as f32 + 1.0) * node_size * 2.0,
            );
            current_node = current_node + 1;
            log!("node: {}, pos: {:?}", node.label, node.pos);
            if current_size == current_node {
                current_node = 0;
                current_size = queue.len();
            }
        }

        pub fn drow() {
            todo!();
        }
    }
}

pub trait ToSingleTree {
    fn to_syntax_tree(&self) -> SyntaxTree;
    fn label(&self) -> String;
}