//! The spelling of a card name's Shortened form [CR#201.5c], a rule over the
//! name alone.

use crate::Numeral;
use crate::NumeralCodec;

// The titles Oracle text keeps when it shortens a name to its title and last
// word, as Captain James T. Kirk's text says "Captain Kirk". They are listed
// because the Shortened form is spelled from the name alone, and nothing in a
// name's words shows which one is a title.
const SURNAME_TITLES: [&str; 5] = ["Captain", "Commander", "Dr.", "Ensign", "Lieutenant"];

/// The spelling of the Shortened form of the card name `name`, or `None`
/// where the name has none.
///
/// - A name whose words before any comma start with a listed title shortens to
///   that title and the last of those words (*Captain Kirk* for *Captain James
///   T. Kirk*); a title and one word are already that (*Dr. Eggman*).
/// - Else a name with a comma shortens to the words before it (*Dina* for
///   *Dina, Soul Steeper*).
/// - A name with no comma has no Shortened form when it starts with *The*.
/// - Else a trailing Roman numeral is dropped (*King Darien* for *King Darien
///   XLVIII*); else the name is cut before its earliest *and the*, *the* or
///   *of* (*Tor Wauki* for *Tor Wauki the Younger*); else it is its first word
///   (*Nissa* for *Nissa Revane*).
/// - A cut of a name with no comma that ends in a possessive word, or just
///   before one, is no Shortened form (*Gaea's* of *Gaea's Cradle*, *Marit* of
///   *Marit Lage's Slumber*): the possessive phrase is one part of a
///   description, and no part of it names the object. A possessive later in the
///   name leaves the cut standing (*Krydle* for *Krydle of Baldur's Gate*).
///
/// Whether a card has a Shortened Name at all is not the spelling's
/// question: it is the Card Scope's ([`crate::CardScope`]).
#[must_use]
pub fn shortened_spelling(name: &str) -> Option<String> {
    let comma = name.split_once(',').map(|(prefix, _)| prefix.trim());
    let base = comma.unwrap_or(name);
    let words: Vec<&str> = base.split(' ').collect();
    let candidate = if let [title, between @ .., surname] = words.as_slice()
        && SURNAME_TITLES.contains(title)
    {
        // The words elided between title and surname hold no possessive.
        if comma.is_none() && between.iter().chain([surname]).any(|word| possessive(word)) {
            return None;
        }
        format!("{title} {surname}")
    } else if let Some(prefix) = comma {
        prefix.to_owned()
    } else if name.starts_with("The ") {
        return None;
    } else if let Some((prefix, last)) = name.rsplit_once(' ')
        && Numeral::Roman.parse(last).is_ok()
    {
        prefix.trim_end().to_owned()
    } else if let Some(index) = [" and the ", " the ", " of "]
        .into_iter()
        .filter_map(|separator| name.find(separator))
        .min()
    {
        name[..index].to_owned()
    } else {
        name.split(' ').next()?.to_owned()
    };
    if comma.is_none()
        && let Some(rest) = name.strip_prefix(candidate.as_str())
    {
        let last = candidate.rsplit(' ').next();
        let next = rest.trim_start().split(' ').next();
        if last.into_iter().chain(next).any(possessive) {
            return None;
        }
    }
    (candidate != name && !candidate.is_empty()).then_some(candidate)
}

/// The spelling of the card name `name` in `form`, or `None` for the
/// Shortened form of a name that has none.
#[must_use]
pub fn own_name_spelling(name: &str, form: crate::NameForm) -> Option<String> {
    match form {
        crate::NameForm::Full => Some(name.to_owned()),
        crate::NameForm::Shortened => shortened_spelling(name),
    }
}

/// Whether a word of a name is a genitive: it ends in *'s* or, after a
/// plural's *s*, a bare apostrophe.
fn possessive(word: &str) -> bool {
    ["'s", "’s", "s'", "s’"]
        .iter()
        .any(|suffix| word.ends_with(suffix))
}

#[cfg(test)]
mod tests {
    use super::shortened_spelling;

    #[test]
    fn a_name_shortens_to_its_comma_prefix_or_its_leading_words() {
        for (name, expected) in [
            ("Gut, True Soul Zealot", Some("Gut")),
            ("The Balrog, Durin's Bane", Some("The Balrog")),
            ("Elspeth, Sun's Champion", Some("Elspeth")),
            ("The First Sliver", None),
            ("King Darien XLVIII", Some("King Darien")),
            ("Grand Arbiter Augustin IV", Some("Grand Arbiter Augustin")),
            ("Tor Wauki the Younger", Some("Tor Wauki")),
            ("Sidar Jabari of Zhalfir", Some("Sidar Jabari")),
            ("Nissa Revane", Some("Nissa")),
            ("Shang-Chi and the Ten Rings", Some("Shang-Chi")),
            ("Abuelo the Brave", Some("Abuelo")),
            ("Progenitus", None),
        ] {
            assert_eq!(shortened_spelling(name).as_deref(), expected, "{name}");
        }
    }

    #[test]
    fn a_cut_at_a_possessive_is_no_shortened_form() {
        for name in [
            "Urza's Saga",
            "Gaea's Cradle",
            "Marit Lage's Slumber",
            "Captain America's Shield",
        ] {
            assert_eq!(shortened_spelling(name), None, "{name}");
        }
        // A possessive later in the name leaves the cut standing.
        assert_eq!(
            shortened_spelling("Krydle of Baldur's Gate").as_deref(),
            Some("Krydle")
        );
        // An apostrophe inside a word marks no genitive.
        assert_eq!(
            shortened_spelling("Sek'Kuar, Deathkeeper").as_deref(),
            Some("Sek'Kuar")
        );
    }

    #[test]
    fn a_listed_title_shortens_to_title_and_surname() {
        for (name, expected) in [
            ("Captain James T. Kirk", Some("Captain Kirk")),
            // A quoted nickname is one more word between title and surname.
            ("Dr. Leonard \"Bones\" McCoy", Some("Dr. McCoy")),
            // The words counted are the name's before its comma.
            ("Ensign Harry Kim, Operations", Some("Ensign Kim")),
            ("Captain Kirk, Boldly Going", Some("Captain Kirk")),
            // A title and a surname are the whole name, never a bare title.
            ("Dr. Eggman", None),
            ("Captain Sisay", None),
            // An unlisted title is cut as any other word.
            ("General Leo Cristophe", Some("General")),
            ("General Kudro of Drannith", Some("General Kudro")),
        ] {
            assert_eq!(shortened_spelling(name).as_deref(), expected, "{name}");
        }
    }
}
