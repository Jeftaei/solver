pub mod args;
pub mod solver;

use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, Default)]
pub enum Sorting {
    #[default]
    None,

    Alphabetical,

    Length,
}

impl std::fmt::Display for Sorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sorting::None => write!(f, "None"),
            Sorting::Alphabetical => write!(f, "Alphabetical"),
            Sorting::Length => write!(f, "Length"),
        }
    }
}
