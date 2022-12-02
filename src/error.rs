/// This module contains the error types for the library.


/// Unvalid parentesis Error
/// This error is thrown when the parentesis are not valid
/// Example: (a|b) | (c|d) is valid
/// Example: (a|b | (c|d) is not valid
/// 

#[derive(Debug)]
pub struct UnvalidParentesis {}
impl std::error::Error for UnvalidParentesis {}
impl std::fmt::Display for UnvalidParentesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unvalid parentesis error")
    }
}

/// Invalid character Error
/// A character is valid when it is a letter or a number or special regex character
/// Or parentesis
/// Example: a is valid
/// Example: 1 is valid
/// Example: * is valid
/// Example: ( is valid
/// Example: ) is valid
/// Example: | is valid
/// Example: + is valid
/// Example: - is not valid

#[derive(Debug)]
pub struct InvalidCharacter {
    pub character: char,
}

impl InvalidCharacter {
    pub fn new(character: char) -> Self {
        Self { character }
    }
}

impl std::error::Error for InvalidCharacter {}

impl std::fmt::Display for InvalidCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid character error: {}", self.character)
    }
}

#[derive(Debug)]
pub struct InvalidTokenError {
    pub message: String,
}

impl InvalidTokenError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::error::Error for InvalidTokenError {}

impl std::fmt::Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token error: {}", self.message)
    }
}