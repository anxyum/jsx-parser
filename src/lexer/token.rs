#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenAngle,  // <
    CloseAngle, // >
    Slash,      // /
    Equals,     // =
    Identifier(String),
    String(String),
    Text(String),
    WhiteSpace,
}
