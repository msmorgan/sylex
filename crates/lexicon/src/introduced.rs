//! The shape of an Introduced Name: the title run a text writes where it
//! gives an object a name [CR#111.4].
//!
//! No inventory lists these names, so the run itself is the name. Lexical
//! analysis proposes one run at each capitalized word, and the grammar
//! admits it only where a name is introduced or, in the same text, named
//! again.

/// The lowercase words a title run continues over (*Scion of the Deep*,
/// *Friend to Elves*). A run stops before any other lowercase word, a
/// coordinator among them: in *named The Mightstone and Weakstone*, *and*
/// may join two names.
const TITLE_PARTICLES: [&str; 3] = ["of", "the", "to"];

/// Whether a title run may start at `start`: at a capital letter that begins
/// a word.
pub(crate) fn starts_title_run(tokens: &[char], start: usize) -> bool {
    tokens[start].is_uppercase() && !inside_word(tokens, start)
}

/// Whether the char at `at` continues a word begun before it: after a letter
/// or digit, or after an apostrophe, hyphen or period that follows one
/// (*Do'Urden*, *Rok-Tahk*, *U.S.Agent*).
fn inside_word(tokens: &[char], at: usize) -> bool {
    let Some(before) = at.checked_sub(1).map(|b| tokens[b]) else {
        return false;
    };
    before.is_alphanumeric()
        || (matches!(before, '\'' | '-' | '.') && at >= 2 && tokens[at - 2].is_alphanumeric())
}

/// The end of the word of a title run that starts at `at`: a letter or
/// digit, then letters, digits, apostrophes and hyphens, and a period only
/// before more of them (*Heroes'*, *Rok-Tahk*, *U.S.Agent*). `None` where no
/// word starts.
fn word_end(tokens: &[char], at: usize) -> Option<usize> {
    let inner = |ch: char| ch.is_alphanumeric() || matches!(ch, '_' | '\'' | '-');
    if !tokens.get(at).is_some_and(|ch| ch.is_alphanumeric()) {
        return None;
    }
    let mut end = at + 1;
    loop {
        match tokens.get(end) {
            Some(&ch) if inner(ch) => end += 1,
            Some('.') if tokens.get(end + 1).is_some_and(|&ch| inner(ch)) => end += 2,
            _ => return Some(end),
        }
    }
}

/// The end of the longest title run that starts at `start`, or `None`
/// where no capitalized word starts there.
///
/// The run continues over capitalized words and numerals, and over the
/// lowercase [`TITLE_PARTICLES`] where a capitalized word follows them. It
/// continues over a comma before a capitalized word or *the* (*Voja, Friend
/// to Elves*; *Ashaya, the Awoken World*), unless `card_name` spells the run
/// so far: a card's name written out ends there (*Equipment named Helm of
/// Kaldra, Sword of Kaldra, and Shield of Kaldra*; *named Skoa, Embermage,
/// Sacrifice two Mountains*). A genitive mark that ends the run is the
/// clitic after the name, no part of it (*Beau's power*).
pub(crate) fn title_run(
    tokens: &[char],
    start: usize,
    card_name: impl Fn(&[char]) -> bool,
) -> Option<usize> {
    let mut end = None;
    let mut at = start;
    while let Some(word) = word_end(tokens, at) {
        if tokens[at].is_uppercase() || tokens[at].is_numeric() {
            end = Some(word);
        } else if at == start
            || !TITLE_PARTICLES
                .iter()
                .any(|particle| particle.chars().eq(tokens[at..word].iter().copied()))
        {
            break;
        }
        let rest = &tokens[word..];
        let capital_after = |skip: usize| rest.get(skip).is_some_and(|ch| ch.is_uppercase());
        if rest.starts_with(&[',', ' '])
            && (capital_after(2)
                || (rest[2..].starts_with(&['t', 'h', 'e', ' ']) && capital_after(6)))
            && !card_name(&tokens[start..word])
        {
            at = word + 2;
        } else if rest.first() == Some(&' ') {
            at = word + 1;
        } else {
            break;
        }
    }
    let mut end = end?;
    if tokens[start..end].ends_with(&['\'', 's']) {
        end -= 2;
    } else if tokens[end - 1] == '\'' {
        end -= 1;
    }
    (end > start).then_some(end)
}

/// Whether `name` has the shape of an Introduced Name: it is a whole title
/// run, as analysis would read it where it stood alone.
pub(crate) fn valid_introduced_name(name: &str) -> bool {
    let tokens: Vec<char> = name.chars().collect();
    !tokens.is_empty()
        && starts_title_run(&tokens, 0)
        && title_run(&tokens, 0, |_| false) == Some(tokens.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(text: &str, start: &str) -> Option<String> {
        run_with(text, start, &[])
    }

    fn run_with(text: &str, start: &str, card_names: &[&str]) -> Option<String> {
        let tokens: Vec<char> = text.chars().collect();
        let at = text.find(start).map(|byte| text[..byte].chars().count())?;
        assert!(starts_title_run(&tokens, at));
        let card_name = |run: &[char]| {
            let run: String = run.iter().collect();
            card_names.contains(&run.as_str())
        };
        title_run(&tokens, at, card_name).map(|end| tokens[at..end].iter().collect())
    }

    #[test]
    fn a_title_run_is_the_longest_run_of_capitalized_words() {
        // Rinoa Heartilly.
        let text = "create Angelo, a legendary 1/1 green and white Dog creature token.";
        assert_eq!(run(text, "Angelo").as_deref(), Some("Angelo"));
        // Kiora, the Rising Tide; Tuktuk the Explorer.
        assert_eq!(
            run("create Scion of the Deep, a legendary 8/8", "Scion").as_deref(),
            Some("Scion of the Deep")
        );
        assert_eq!(
            run("create Tuktuk the Returned, a legendary", "Tuktuk").as_deref(),
            Some("Tuktuk the Returned")
        );
        // A particle no capitalized word follows ends the run before it.
        assert_eq!(
            run("named Cadet to the battlefield", "Cadet").as_deref(),
            Some("Cadet")
        );
        // Urza, Lord Protector: a coordinator ends the run.
        assert_eq!(
            run("an artifact named The Mightstone and Weakstone", "The").as_deref(),
            Some("The Mightstone")
        );
    }

    #[test]
    fn a_comma_inside_a_name_continues_the_run() {
        // Tolsimir, Friend to Wolves; Nissa, Sage Animist; The Irencrag.
        for (text, name) in [
            (
                "create Voja, Friend to Elves, a legendary 3/3",
                "Voja, Friend to Elves",
            ),
            (
                "create Ashaya, the Awoken World, a legendary",
                "Ashaya, the Awoken World",
            ),
            (
                "named Everflame, Heroes' Legacy and attach it",
                "Everflame, Heroes' Legacy",
            ),
        ] {
            let start = name.split(',').next().unwrap();
            assert_eq!(run(text, start).as_deref(), Some(name));
        }
    }

    #[test]
    fn a_card_name_written_out_ends_the_run_at_its_comma() {
        // Skoa, Embermage.
        let text = "Discard another card named Skoa, Embermage, Sacrifice two Mountains";
        assert_eq!(
            run_with(text, "Skoa", &["Skoa, Embermage"]).as_deref(),
            Some("Skoa, Embermage")
        );
        // Helm of Kaldra.
        let text = "Equipment named Helm of Kaldra, Sword of Kaldra, and Shield of Kaldra";
        assert_eq!(
            run_with(text, "Helm", &["Helm of Kaldra"]).as_deref(),
            Some("Helm of Kaldra")
        );
    }

    #[test]
    fn a_final_genitive_mark_is_no_part_of_the_run() {
        // Bonny Pall, Clearcutter; Mishra, Eminent One.
        assert_eq!(
            run("\"Beau's power is equal", "Beau").as_deref(),
            Some("Beau")
        );
        assert_eq!(
            run("its name is Mishra's Warform and", "Mishra").as_deref(),
            Some("Mishra's Warform")
        );
    }

    #[test]
    fn a_run_starts_only_at_a_capital_that_begins_a_word() {
        let tokens: Vec<char> = "Drizzt Do'Urden and Rok-Tahk".chars().collect();
        let starts: Vec<usize> = (0..tokens.len())
            .filter(|&at| starts_title_run(&tokens, at))
            .collect();
        assert_eq!(starts, [0, 7, 20]);
    }

    #[test]
    fn an_introduced_name_is_a_whole_title_run() {
        for name in [
            "Angelo",
            "Voja, Friend to Elves",
            "Mishra's Warform",
            "Tuktuk the Returned",
        ] {
            assert!(valid_introduced_name(name), "{name}");
        }
        for name in [
            "",
            "angelo",
            "Angelo and Boo",
            "Beau's",
            "Angelo ",
            "Angelo, a Dog",
        ] {
            assert!(!valid_introduced_name(name), "{name}");
        }
    }
}
