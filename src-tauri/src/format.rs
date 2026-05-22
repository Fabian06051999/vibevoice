use crate::context::{RecordingContext, VibeMode};

pub fn format_transcript(text: &str, context: &RecordingContext) -> String {
    let mode = refine_mode(context.mode, text);
    let mut result = match mode {
        VibeMode::Code => apply_replacements(text, CODE_REPLACEMENTS),
        VibeMode::Terminal => apply_replacements(text, TERMINAL_REPLACEMENTS),
        VibeMode::Prompt | VibeMode::Prose => text.trim().to_string(),
    };

    if mode == VibeMode::Code {
        result = apply_replacements(&result, CODE_PRESETS);
    }

    if matches!(mode, VibeMode::Code | VibeMode::Terminal) {
        result = tidy_symbol_spacing(&result);
    }

    result.trim().to_string()
}

fn tidy_symbol_spacing(text: &str) -> String {
    text.replace(" (", "(")
        .replace(" {", "{")
        .replace(" [", "[")
        .replace(" .", ".")
        .replace(" ,", ",")
        .replace(" ;", ";")
        .replace(" :", ":")
}

pub fn refine_mode(mode: VibeMode, text: &str) -> VibeMode {
    let normalized = text.trim().to_lowercase();

    if normalized.starts_with("git ")
        || normalized.starts_with("npm ")
        || normalized.starts_with("cargo ")
        || normalized.starts_with("docker ")
        || normalized.starts_with("cd ")
        || normalized.starts_with("pnpm ")
        || normalized.starts_with("yarn ")
        || normalized.starts_with("python ")
        || normalized.starts_with("pip ")
    {
        return VibeMode::Terminal;
    }

    mode
}

fn apply_replacements(text: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = text.to_string();
    for (from, to) in replacements {
        result = replace_phrase(&result, from, to);
    }
    result
}

fn replace_phrase(text: &str, phrase: &str, replacement: &str) -> String {
    if phrase.is_empty() {
        return text.to_string();
    }

    let lower_text = text.to_lowercase();
    let lower_phrase = phrase.to_lowercase();
    let mut output = String::with_capacity(text.len());
    let mut index = 0;

    while index < text.len() {
        if lower_text[index..].starts_with(&lower_phrase) {
            output.push_str(replacement);
            index += phrase.len();
        } else if let Some(ch) = text[index..].chars().next() {
            output.push(ch);
            index += ch.len_utf8();
        } else {
            break;
        }
    }

    output
}

const CODE_REPLACEMENTS: &[(&str, &str)] = &[
    ("geschweifte klammer auf", "{"),
    ("geschweifte klammer zu", "}"),
    ("eckige klammer auf", "["),
    ("eckige klammer zu", "]"),
    ("klammer auf", "("),
    ("klammer zu", ")"),
    ("open brace", "{"),
    ("close brace", "}"),
    ("open bracket", "["),
    ("close bracket", "]"),
    ("open paren", "("),
    ("close paren", ")"),
    ("open parenthesis", "("),
    ("close parenthesis", ")"),
    ("pfeil", "=>"),
    ("arrow", "=>"),
    ("fat arrow", "=>"),
    ("doppelpunkt doppelpunkt", "::"),
    ("double colon", "::"),
    ("gleich gleich gleich", "==="),
    ("triple equals", "==="),
    ("gleich gleich", "=="),
    ("double equals", "=="),
    ("strikter gleich", "==="),
    ("strict equal", "==="),
    ("nicht gleich", "!="),
    ("not equal", "!="),
    ("größer gleich", ">="),
    ("greater equal", ">="),
    ("kleiner gleich", "<="),
    ("less equal", "<="),
    ("und und", "&&"),
    ("oder oder", "||"),
    ("semikolon", ";"),
    ("semicolon", ";"),
    ("doppelpunkt", ":"),
    ("colon", ":"),
    ("komma", ","),
    ("comma", ","),
    ("punkt", "."),
    ("period", "."),
    ("dot", "."),
    ("plus gleich", "+="),
    ("minus gleich", "-="),
    ("new line", "\n"),
    ("newline", "\n"),
    ("neue zeile", "\n"),
    ("zeilenumbruch", "\n"),
    ("backtick", "`"),
    ("template string", "`"),
];

const TERMINAL_REPLACEMENTS: &[(&str, &str)] = &[
    ("pipe", "|"),
    ("rohr", "|"),
    ("klammer auf", "("),
    ("klammer zu", ")"),
    ("open paren", "("),
    ("close paren", ")"),
    ("doppelpunkt doppelpunkt", "::"),
    ("double colon", "::"),
    ("gleich gleich", "=="),
    ("double equals", "=="),
    ("semikolon", ";"),
    ("semicolon", ";"),
    ("doppelpunkt", ":"),
    ("colon", ":"),
    ("new line", "\n"),
    ("newline", "\n"),
    ("neue zeile", "\n"),
    ("backslash", "\\"),
    ("forward slash", "/"),
    ("slash", "/"),
];

const CODE_PRESETS: &[(&str, &str)] = &[
    ("use effect", "useEffect"),
    ("use state", "useState"),
    ("use ref", "useRef"),
    ("use memo", "useMemo"),
    ("use callback", "useCallback"),
    ("use context", "useContext"),
    ("use reducer", "useReducer"),
    ("react query", "React Query"),
    ("next js", "Next.js"),
    ("tailwind", "Tailwind"),
    ("typescript", "TypeScript"),
    ("javascript", "JavaScript"),
    ("console log", "console.log"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_spoken_code_symbols() {
        let context = RecordingContext {
            mode: VibeMode::Code,
            language_hint: Some("typescript".to_string()),
        };
        assert_eq!(
            format_transcript("use effect klammer auf", &context),
            "useEffect("
        );
    }

    #[test]
    fn converts_terminal_pipes() {
        let context = RecordingContext {
            mode: VibeMode::Terminal,
            language_hint: None,
        };
        assert_eq!(
            format_transcript("git status pipe grep test", &context),
            "git status | grep test"
        );
    }

    #[test]
    fn refines_terminal_from_content() {
        assert_eq!(
            refine_mode(VibeMode::Code, "npm run dev"),
            VibeMode::Terminal
        );
    }
}
