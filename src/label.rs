pub enum SgLabel {
    /// Lexical parent edge
    Parent,
    /// Local declaration
    Declaration,
}


pub enum ModuleLabel {
    /// Lexical parent edge
    Parent,
    /// Import edge
    Import,
    /// Local declaration
    Declaration,
}