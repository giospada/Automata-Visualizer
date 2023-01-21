
pub const EPSILON: char = 'ε';
pub const STRING_END: char = '$';

// NOTE: this could be in conflict with the Terminal symbols, so
// it is MANDATORY that the Terminal doesn´t have dots in it!
pub const ITEM_SEP: char = '.';

pub type NonTerminal = usize;
pub type Terminal = char;
