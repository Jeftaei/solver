use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Prompt to solve for
    pub prompt: String,

    #[arg(short = 'o', long = "output")]
    /// Write output to a file (will create a file if it doesn't exist)
    pub file: Option<std::path::PathBuf>,

    #[arg(short = 'd', long = "dictionary")]
    /// Path to a .txt file containing words separated by newlines
    pub dictionary: Option<std::path::PathBuf>,

    #[arg(short = 's', long = "sort")]
    /// Sort the output by length or alphabetically (default: None)
    pub sorting: Option<crate::modules::Sorting>,
}

// small workaround to avoid having `use clap::Parser;` in main.rs
impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
