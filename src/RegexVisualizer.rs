use crate::SyntaxTree::*;
use crate::RegularExpression::*;
use egui::{Pos2};


pub struct RegexVisualizer{
    pub tree: Option<SyntaxTree>,
    pub regex_text: String,
    pub regex_error:Option<String>,
    pub size_node:f32,
    pub padding_y:f32,
    pub padding_x:f32,
    pub open:bool
}

impl RegexVisualizer{
    pub fn new() -> Self{
        Self{
            tree:None,
            regex_text:String::new(),
            regex_error:None,
            open:false,
            padding_x:20.,
            padding_y:20.,
            size_node:20.
        }
    }

    pub fn check_open(&mut self){
        if let None = self.tree{
            self.open=false;
        }
        if !self.open{
            self.tree=None;
        }
    }


    pub fn generate_tree(&mut self){
        match ReOperator::from_string(&self.regex_text){
            Ok(tree) => {
                self.tree = Some(tree.to_syntax_tree());
                self.regex_error=None;
                self.open=true;
            },
            Err(err) =>{
                self.tree=None;
                self.regex_error=Some(err.to_string());
                self.open=false;
            }
        };
    }
}
