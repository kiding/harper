use crate::{
    Token, TokenStringExt,
    patterns::{Pattern, SequencePattern},
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct DespiteOf {
    pattern: Box<dyn Pattern>,
}

impl Default for DespiteOf {
    fn default() -> Self {
        let pattern = SequencePattern::aco("despite")
            .then_whitespace()
            .then_exact_word("of");

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for DespiteOf {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched: &[Token], source: &[char]) -> Option<Lint> {
        let span = matched.span()?;
        let matched = span.get_content(source);

        Some(Lint {
            span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![
                Suggestion::replace_with_match_case_str("despite", matched),
                Suggestion::replace_with_match_case_str("in spite of", matched)
            ],
            message: "The phrase “despite of” is incorrect. Please use either “despite” or “in spite of” instead.".to_string(),
            priority: 126,
        })
    }

    fn description(&self) -> &'static str {
        "Corrects the misuse of `despite of` and suggests the proper alternatives `despite` or `in spite of`."
    }
}

#[cfg(test)]
mod tests {
    use super::DespiteOf;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn catches_lowercase() {
        assert_suggestion_result(
            "The team performed well, despite of the difficulties they faced.",
            DespiteOf::default(),
            "The team performed well, despite the difficulties they faced.",
        );
    }

    #[test]
    fn catches_different_cases() {
        assert_lint_count(
            "Despite of the rain, we went for a walk.",
            DespiteOf::default(),
            1,
        );
    }

    #[test]
    fn likes_correction() {
        assert_lint_count(
            "The team performed well, despite the difficulties they faced. In spite of the rain, we went for a walk.",
            DespiteOf::default(),
            0,
        );
    }
}
