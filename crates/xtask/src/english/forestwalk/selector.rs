//! The fork selectors of `forestwalk --pick` and `--exclude`: one argument
//! names a fork and one of its options as the survey prints them, so a fork
//! header and an option copied from the survey's output are a selector.
//!
//! ```text
//! KIND "surface"#N ANSWER
//! ```
//!
//! `KIND` is `lexical`, `dependency` or `frame`; the word is `"surface"#N`,
//! its surface and 1-based occurrence, the occurrence always written
//! ([`ForkWord`]); `ANSWER` is the option as an option line prints it, without
//! its count or any ` = ` alias answer (an alias's answer for an alias's
//! name). The kind and word end at
//! the occurrence, so the answer is everything after it: it need not be parsed,
//! and is matched against the answers the survey prints for that question.
//!
//! Selectors are kept apart from the `=>` assertions of a judgment: they begin
//! with a lowercase kind and name grammar child names (`adjunct`, `first`),
//! where an assertion begins with a Category and names CGEL functions.

use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Args;
use clap::Command;
use clap::FromArgMatches;
use sylex_syntax::Answer;
use sylex_syntax::Fork;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;

/// How a selector narrows the forest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Narrowing {
    /// Keep the Derivations that give the option.
    Pick,
    /// Drop the Derivations that give the option.
    Exclude,
}

impl Narrowing {
    pub(super) fn flag(self) -> &'static str {
        match self {
            Self::Pick => "--pick",
            Self::Exclude => "--exclude",
        }
    }
}

/// The `--pick` and `--exclude` arguments in the order they were written:
/// narrowing steps do not commute in the counts printed after each, nor in
/// the error for an option an earlier step removed.
#[derive(Debug, Default)]
pub(super) struct Steps(Vec<(Narrowing, String)>);

impl Steps {
    /// The steps' selectors, each read.
    ///
    /// # Errors
    /// The first selector that is not written `KIND "surface"#N ANSWER`.
    pub(super) fn selectors(&self) -> Result<Vec<Selector>> {
        self.0
            .iter()
            .map(|(narrowing, text)| Selector::parse(*narrowing, text))
            .collect()
    }
}

impl Args for Steps {
    fn augment_args(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("pick")
                .long("pick")
                .value_name("SELECTOR")
                .action(ArgAction::Append)
                .help("Keep the Derivations that give a fork's option (repeatable)")
                .long_help(
                    "Keep the Derivations that give a fork's option (repeatable). A selector is \
                     one argument, `KIND \"surface\"#N ANSWER`: a fork's name as the survey \
                     prints it (an alias's too) and one option's answer after it, without its \
                     count or any ` = ` alias answer (an alias's answer for an alias's name), as \
                     in `--pick 'dependency \"from\"#1 modifier of Nominal \"spells\"#1'`. Steps \
                     apply in the order written, `--pick` and `--exclude` alike.",
                ),
        )
        .arg(
            Arg::new("exclude")
                .long("exclude")
                .value_name("SELECTOR")
                .action(ArgAction::Append)
                .help("Drop the Derivations that give a fork's option (repeatable)")
                .long_help(
                    "Drop the Derivations that give a fork's option (repeatable). A selector is \
                     spelled as for `--pick`.",
                ),
        )
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        Self::augment_args(cmd)
    }
}

impl FromArgMatches for Steps {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let mut steps: Vec<(usize, Narrowing, String)> = Vec::new();
        for (id, narrowing) in [("pick", Narrowing::Pick), ("exclude", Narrowing::Exclude)] {
            let values = matches.get_many::<String>(id).into_iter().flatten();
            let indices = matches.indices_of(id).into_iter().flatten();
            steps.extend(
                indices
                    .zip(values)
                    .map(|(index, value)| (index, narrowing, value.clone())),
            );
        }
        steps.sort_by_key(|(index, _, _)| *index);
        Ok(Self(
            steps
                .into_iter()
                .map(|(_, narrowing, text)| (narrowing, text))
                .collect(),
        ))
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

/// One selector as written, its answer not yet matched.
#[derive(Debug)]
pub(super) struct Selector {
    narrowing: Narrowing,
    /// The argument, trimmed, for the errors that name it.
    text: String,
    kind: ForkKind,
    /// The word's surface as the survey prints it: Rust's `{:?}`, between its
    /// quotes.
    quoted: String,
    occurrence: usize,
    answer: String,
}

impl Selector {
    fn parse(narrowing: Narrowing, argument: &str) -> Result<Self> {
        let text = argument.trim();
        let flag = narrowing.flag();
        let fail = |why: &str| anyhow::anyhow!("`{flag} {text}`: {why}");
        if text.contains("=>") {
            return Err(fail(
                "this is a judgment assertion; a selector names a fork and one of its options \
                 as the survey prints them: `KIND \"surface\"#N ANSWER`",
            ));
        }
        let (kind_word, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
        let kind = match kind_word {
            "lexical" => ForkKind::Lexical,
            "dependency" => ForkKind::Dependency,
            "frame" => ForkKind::Frame,
            category if category.starts_with(char::is_uppercase) => {
                return Err(fail(&format!(
                    "`{category}` begins a judgment assertion, not a selector; a selector begins \
                     with `lexical`, `dependency` or `frame` and names a fork and one of its \
                     options as the survey prints them: `KIND \"surface\"#N ANSWER`"
                )));
            }
            other => {
                return Err(fail(&format!(
                    "unknown kind `{other}`; a selector begins with `lexical`, `dependency` or \
                     `frame`"
                )));
            }
        };
        let Some(body) = rest.trim_start().strip_prefix('"') else {
            return Err(fail(
                "the kind is followed by the fork's word, quoted: `KIND \"surface\"#N ANSWER`",
            ));
        };
        let mut chars = body.char_indices();
        let mut end = None;
        while let Some((at, c)) = chars.next() {
            match c {
                '\\' => {
                    chars.next();
                }
                '"' => {
                    end = Some(at);
                    break;
                }
                _ => {}
            }
        }
        let Some(end) = end else {
            return Err(fail("the quote opening the word is never closed"));
        };
        let quoted = body[..end].to_owned();
        let Some(digits) = body[end + 1..].strip_prefix('#') else {
            return Err(fail(&format!(
                "the word \"{quoted}\" needs its occurrence: write `\"surface\"#N`, N counting \
                 from 1, for a surface that occurs once too"
            )));
        };
        let digits_end = digits
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(digits.len());
        let (number, answer) = digits.split_at(digits_end);
        if number.is_empty() || !(answer.is_empty() || answer.starts_with(char::is_whitespace)) {
            return Err(fail(&format!(
                "the word \"{quoted}\" needs its occurrence as digits after `#`: \
                 `\"surface\"#N`"
            )));
        }
        let occurrence: usize = number
            .parse()
            .map_err(|_| fail(&format!("the occurrence `#{number}` is not a count")))?;
        if occurrence == 0 {
            return Err(fail("occurrences count from 1: `#1` is a surface's first"));
        }
        let answer = answer.trim();
        if answer.is_empty() {
            return Err(fail(
                "no option is named: write the option as the survey prints it, after the word",
            ));
        }
        if kind == ForkKind::Dependency && answer.ends_with('"') {
            return Err(fail(
                "the governor word needs its occurrence: write it `\"surface\"#N`, as the \
                 survey prints it",
            ));
        }
        Ok(Self {
            narrowing,
            text: text.to_owned(),
            kind,
            quoted,
            occurrence,
            answer: answer.to_owned(),
        })
    }
}

/// A question the survey prints a fork or alias header for, with the answers
/// it prints for its options.
#[derive(Debug)]
pub(super) struct Question {
    kind: ForkKind,
    word: ForkWord,
    options: Vec<(String, Answer)>,
}

/// The questions of a unit's forks: each fork's own and each alias, with
/// every option's answer as the survey prints it.
///
/// # Errors
/// Two options of one question that print alike, which would make a selector
/// ambiguous.
pub(super) fn questions(forks: &[Fork]) -> Result<Vec<Question>> {
    let mut out = Vec::new();
    for fork in forks {
        let mut own = Vec::new();
        let mut aliases: Vec<Vec<(String, Answer)>> = vec![Vec::new(); fork.aliases.len()];
        for option in &fork.options {
            own.push((super::answer(&option.answer), option.answer.clone()));
            for (printed, alias) in aliases.iter_mut().zip(&option.aliases) {
                printed.push((super::answer(alias), alias.clone()));
            }
        }
        let mut asked = vec![(fork.kind, fork.word.clone(), own)];
        asked.extend(
            fork.aliases
                .iter()
                .zip(aliases)
                .map(|((kind, word), options)| (*kind, word.clone(), options)),
        );
        for (kind, word, options) in asked {
            for (at, (printed, _)) in options.iter().enumerate() {
                anyhow::ensure!(
                    options[..at].iter().all(|(other, _)| other != printed),
                    "two options of {} print alike, `{printed}`, so a selector could not tell \
                     them apart",
                    super::fork_name(kind, &word)
                );
            }
            out.push(Question {
                kind,
                word,
                options,
            });
        }
    }
    Ok(out)
}

/// A selector matched to a fork's option of the unit.
#[derive(Debug)]
pub(super) struct Resolved {
    pub(super) narrowing: Narrowing,
    /// The selector as written, for errors.
    pub(super) text: String,
    pub(super) kind: ForkKind,
    pub(super) word: ForkWord,
    pub(super) answer: Answer,
    /// The fork and option as the survey prints them.
    pub(super) printed: String,
}

impl Selector {
    /// The question and option of the unit this selector names. `words` are
    /// the unit's words and `questions` those of its forks, read before any
    /// narrowing: whether an earlier step has since removed the option is
    /// for the narrowing to say.
    ///
    /// # Errors
    /// A word the unit lacks, a (kind, word) that is no fork or alias of it,
    /// or an answer that is no option of that question.
    pub(super) fn resolve(&self, words: &[ForkWord], questions: &[Question]) -> Result<Resolved> {
        let flag = self.narrowing.flag();
        let text = &self.text;
        let Some(word) = words.iter().find(|w| {
            format!("{:?}", w.surface) == format!("\"{}\"", self.quoted)
                && w.occurrence == self.occurrence
        }) else {
            let seen = words
                .iter()
                .filter(|w| format!("{:?}", w.surface) == format!("\"{}\"", self.quoted))
                .count();
            let known = match seen {
                0 => format!("\"{}\" is no word of the unit", self.quoted),
                n => format!("\"{}\" occurs {n} times in the unit", self.quoted),
            };
            anyhow::bail!(
                "`{flag} {text}`: the unit has no word \"{}\"#{}: {known}",
                self.quoted,
                self.occurrence
            );
        };
        let name = super::fork_name(self.kind, word);
        let Some(question) = questions
            .iter()
            .find(|q| q.kind == self.kind && q.word.names(word))
        else {
            let forks: Vec<String> = questions
                .iter()
                .map(|q| super::fork_name(q.kind, &q.word))
                .collect();
            anyhow::bail!(
                "`{flag} {text}`: {name} is no fork or alias of the unit; its forks and \
                 aliases: {}",
                if forks.is_empty() { "none".to_owned() } else { forks.join(", ") }
            );
        };
        let Some((printed, answer)) = question.options.iter().find(|(p, _)| *p == self.answer)
        else {
            if self.answer.contains(" = ")
                && let Some(single) = self
                    .answer
                    .split(" = ")
                    .find(|part| question.options.iter().any(|(p, _)| p == part))
            {
                anyhow::bail!(
                    "`{flag} {text}`: ` = ` separates aliased answers on one survey line; for \
                     {name} alone, write `{single}`"
                );
            }
            let options: Vec<String> = question
                .options
                .iter()
                .map(|(printed, _)| format!("  {printed}"))
                .collect();
            anyhow::bail!(
                "`{flag} {text}`: {name} has no option `{}`; its options:\n{}",
                self.answer,
                options.join("\n")
            );
        };
        Ok(Resolved {
            narrowing: self.narrowing,
            text: self.text.clone(),
            kind: self.kind,
            word: question.word.clone(),
            answer: answer.clone(),
            printed: format!("{name} {printed}"),
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        steps: Steps,
    }

    fn steps(args: &[&str]) -> Vec<(Narrowing, String)> {
        Cli::try_parse_from(std::iter::once("walk").chain(args.iter().copied()))
            .unwrap()
            .steps
            .0
    }

    fn fails(text: &str) -> String {
        Selector::parse(Narrowing::Pick, text)
            .unwrap_err()
            .to_string()
    }

    /// `--pick` and `--exclude` keep the order they were written in, which
    /// clap's own per-flag lists lose.
    #[test]
    fn steps_keep_the_order_they_were_written_in() {
        assert_eq!(
            steps(&[
                "--exclude",
                "a",
                "--pick",
                "b",
                "--exclude",
                "c",
                "--pick",
                "d"
            ]),
            [
                (Narrowing::Exclude, "a".to_owned()),
                (Narrowing::Pick, "b".to_owned()),
                (Narrowing::Exclude, "c".to_owned()),
                (Narrowing::Pick, "d".to_owned()),
            ]
        );
        assert_eq!(steps(&[]).len(), 0);
    }

    /// A fork and an option copied from the survey is one selector: the
    /// word ends at its occurrence, so the answer after it may hold quotes,
    /// colons, commas, braces, `=` and `#`.
    #[test]
    fn a_selector_is_a_kind_a_word_with_its_occurrence_and_an_answer() {
        let read = |text: &str| Selector::parse(Narrowing::Exclude, text).unwrap();
        let dependency = read(" dependency \"as though\"#12 modifier of NounPhrase#And \"and\"#3 ");
        assert_eq!(dependency.kind, ForkKind::Dependency);
        assert_eq!(dependency.quoted, "as though");
        assert_eq!(dependency.occurrence, 12);
        assert_eq!(dependency.answer, "modifier of NounPhrase#And \"and\"#3");
        let lexical = read(
            "lexical \"attached\"#2 action:attach Preterite {finiteness=Finite, person=Third}",
        );
        assert_eq!(lexical.kind, ForkKind::Lexical);
        assert_eq!(
            lexical.answer,
            "action:attach Preterite {finiteness=Finite, person=Third}"
        );
        let frame = read("frame \"white\"#1 frames [Valence:BareNominal/Bare, Unmodified]");
        assert_eq!(frame.kind, ForkKind::Frame);
        assert_eq!(
            frame.answer,
            "frames [Valence:BareNominal/Bare, Unmodified]"
        );
        let escaped = read("lexical \"say \\\"a\\\"\"#1 noun:say Singular");
        assert_eq!(escaped.quoted, "say \\\"a\\\"");
        assert_eq!(read("dependency \"x\"#1 root").answer, "root");
    }

    #[test]
    fn a_malformed_selector_is_an_error_naming_it() {
        assert!(fails("fork \"x\"#1 root").contains("unknown kind `fork`"));
        assert!(fails("").contains("unknown kind ``"));
        assert!(fails("dependency x#1 root").contains("followed by the fork's word, quoted"));
        assert!(fails("dependency \"x#1 root").contains("never closed"));
        let bare = fails("dependency \"x\" root");
        assert!(bare.contains("needs its occurrence"), "{bare}");
        assert!(
            bare.starts_with("`--pick dependency \"x\" root`: "),
            "{bare}"
        );
        assert!(fails("dependency \"x\"#one root").contains("occurrence as digits"));
        assert!(fails("dependency \"x\"#1root").contains("occurrence as digits"));
        assert!(fails("dependency \"x\"#0 root").contains("count from 1"));
        assert!(fails("dependency \"x\"#1").contains("no option is named"));
        assert!(fails("dependency \"x\"#1   ").contains("no option is named"));
        let governor = fails("dependency \"x\"#1 modifier of Nominal \"card\"");
        assert!(
            governor.contains("governor word needs its occurrence"),
            "{governor}"
        );
        let assertion = fails("NounPhrase \"x\" as Object of destroy");
        assert!(
            assertion.contains("begins a judgment assertion"),
            "{assertion}"
        );
        let arrow = fails("dependency \"x\"#1 root => NounPhrase \"x\"");
        assert!(arrow.contains("this is a judgment assertion"), "{arrow}");
    }
}
