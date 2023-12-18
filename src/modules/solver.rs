use crate::modules::Sorting;
use regex::RegexBuilder;

pub struct SolverInfo {
    pub old_prompt: String,
    pub prompt: String,
    pub matches: Vec<String>,
    pub time_taken: std::time::Duration,
    pub sorting: crate::modules::Sorting,
}

impl SolverInfo {
    pub fn get_header(&self) -> String {
        format!(
            "Prompt: {} ({}) | Time taken: {}ms | Matches: {} | Sorting: {}\n\n",
            self.old_prompt.to_uppercase(),
            self.prompt.to_uppercase(),
            self.time_taken.as_nanos() as f64 / 1_000_000_f64,
            self.matches.len(),
            self.sorting
        )
    }
}

pub fn validate_regex(prompt: String) -> String {
    format!(
        "{}{}{}",
        if prompt.trim().starts_with('^') {
            ""
        } else {
            "^.*"
        },
        prompt,
        if prompt.trim().ends_with('$') {
            ""
        } else {
            ".*$"
        }
    )
}

pub fn solve(prompt: String, dictionary: String, sorting: Option<Sorting>) -> SolverInfo {
    let old_prompt = prompt.clone();
    let prompt = validate_regex(prompt);

    let re = match RegexBuilder::new(&prompt)
        .case_insensitive(true)
        .crlf(true)
        .multi_line(true)
        .build()
    {
        Ok(re) => re,
        Err(e) => {
            panic!("Invalid regex: {}", e);
        }
    };

    let start = std::time::Instant::now();
    let mut matches = re
        .find_iter(&dictionary)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();

    if let Some(s) = sorting {
        match s {
            Sorting::Alphabetical => {
                matches.sort_by_key(|a| a.to_lowercase());
            }
            Sorting::Length => {
                matches.sort_by_key(|a| a.len());
            }
            Sorting::None => {}
        }
    }

    let time_taken = start.elapsed();
    SolverInfo {
        old_prompt,
        prompt,
        matches,
        time_taken,
        sorting: crate::modules::Sorting::None,
    }
}
