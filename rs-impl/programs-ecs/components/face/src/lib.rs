use bolt_lang::*;

declare_id!("CT8KSUDE4AUQzNLPyPA5u3CzdWKo6srzRDJrq9dhg3Kv");

#[component]
#[derive(Default)]
pub struct Face {
    #[max_len(20)]
    pub eyes: String,
    #[max_len(20)]
    pub mouth: String,
    #[max_len(20)]
    pub eyebrows: String,
    #[max_len(20)]
    pub description: String,
}
