pub fn format_transcript(text: &str) -> String {
    text.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_prompt_whitespace() {
        assert_eq!(
            format_transcript("  Refactor   this component   \n\n and add tests.  "),
            "Refactor this component\n\nand add tests."
        );
    }

    #[test]
    fn keeps_spoken_prompt_words_intact() {
        assert_eq!(
            format_transcript("use effect open paren"),
            "use effect open paren"
        );
    }
}
