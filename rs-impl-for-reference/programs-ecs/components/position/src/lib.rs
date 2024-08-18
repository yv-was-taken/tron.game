use bolt_lang::*;

declare_id!("2GwJmqFq7ggk9PzqQXnAAAoUVHzeVUberAzdtk15weUR");

#[component]
#[derive(Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    #[max_len(20)]
    pub description: String,
}
