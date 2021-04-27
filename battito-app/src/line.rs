use rustyline_derive::Helper;
use rustyline::highlight::{MatchingBracketHighlighter, Highlighter};
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use rustyline::validate::{Validator, MatchingBracketValidator};
use rustyline::{validate, Context};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::completion::{Completer, Pair, FilenameCompleter};
use rustyline::error::ReadlineError;

#[derive(Helper)]
pub struct MyHelper {
    pub highlighter: MatchingBracketHighlighter,
    pub colored_prompt: String,
    pub completer: FilenameCompleter,
    pub validator: MatchingBracketValidator,
    pub hinter: HistoryHinter,
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MyHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}