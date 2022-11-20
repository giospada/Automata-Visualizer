use crate::SyntaxTree::*;
pub enum ReOperator {
    Char(char),
    Concat(Box<ReOperator>, Box<ReOperator>),
    Or(Box<ReOperator>, Box<ReOperator>),
    KeeneStar(Box<ReOperator>),
}


impl ToSingleTree for ReOperator {
    fn to_syntax_tree(&self) -> SyntaxTree {
        let mut syntax = SyntaxTree::from_label(self.label());
        match self {
            ReOperator::Char(c) => {}
            ReOperator::Concat(left, right) => {
                syntax.children.push(left.to_syntax_tree());
                syntax.children.push(right.to_syntax_tree());
            }
            ReOperator::Or(left, right) => {
                syntax.children.push(left.to_syntax_tree());
                syntax.children.push(right.to_syntax_tree());
            }
            ReOperator::KeeneStar(op) => {
                syntax.children.push(op.to_syntax_tree());
            }
        }
        syntax
    }
    fn label(&self) -> String {
        match self {
            ReOperator::Char(c) => c.to_string(),
            ReOperator::Concat(l, r) => "+".to_string(),
            ReOperator::Or(l, r) => "|".to_string(),
            ReOperator::KeeneStar(r) => "*".to_string(),
        }
    }
}
