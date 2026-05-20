#[derive(Debug)]
pub enum Token {
    OpenAngle,  // <
    CloseAngle, // >
    Slash,      // /
    Equal,      // =
    Identifier(String),
    String(String),
    Text(String),
}
