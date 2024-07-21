use strum_macros::{Display, EnumString};

#[derive(Clone, Copy, Display, PartialEq, EnumString)]
pub enum NpcDialogue {
    Ami,
    Ima,
}
