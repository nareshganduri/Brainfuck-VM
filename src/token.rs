#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    IncDp,
    DecDp,
    Inc,
    Dec,
    Output,
    Input,
    Branch,
    Jump,
    Eof,
}
