#[derive(Debug, Clone, Hash)]
pub enum AST {
    Ident(String),
    Int(isize),
    List(Vec<AST>),
    Str(String),
    Null
}
