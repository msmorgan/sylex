use crate::Category;
use crate::FeatureBundle;
use crate::Finiteness;
use crate::Lexeme;
use crate::Number;
use crate::Person;
use crate::Tense;
use crate::WordForm;

pub(crate) fn applicable(category: Category, form: WordForm, features: &FeatureBundle) -> bool {
    // Each Adjacency Agreement dimension, and the card name's form, belongs
    // to one Category's invariant form.
    for (present, owner) in [
        (features.following_onset.is_some(), Category::Determinative),
        (features.hyphenation.is_some(), Category::Affix),
        (features.genitive.is_some(), Category::Clitic),
        (features.name.is_some(), Category::Catalog),
    ] {
        if present && (category != owner || form != WordForm::Invariant) {
            return false;
        }
    }
    let inflectional = FeatureBundle {
        following_onset: None,
        hyphenation: None,
        genitive: None,
        name: None,
        ..features.clone()
    };
    let features = &inflectional;
    let nominal =
        features.person.is_none() && features.tense.is_none() && features.finiteness.is_none();
    match form {
        WordForm::IndependentGenitive => {
            category == Category::Pronoun
                && features.case == Some(crate::Case::Genitive)
                && features.person.is_some()
                && features.number.is_some()
                && features.tense.is_none()
                && features.finiteness.is_none()
        }
        WordForm::Invariant => match category {
            Category::Verb => false,
            Category::Pronoun => features.tense.is_none() && features.finiteness.is_none(),
            Category::Noun | Category::Determinative => nominal,
            _ => features == &FeatureBundle::default(),
        },
        WordForm::Singular | WordForm::Plural => {
            category == Category::Noun
                && nominal
                && features.number
                    == Some(if form == WordForm::Singular {
                        Number::Singular
                    } else {
                        Number::Plural
                    })
        }
        WordForm::Plain | WordForm::GerundParticiple | WordForm::PastParticiple => {
            category == Category::Verb
                && features.finiteness == (form != WordForm::Plain).then_some(Finiteness::Nonfinite)
                && features.tense.is_none()
                && features.person.is_none()
                && features.number.is_none()
                && features.case.is_none()
        }
        WordForm::Irrealis => {
            category == Category::Verb
                && features.finiteness == Some(Finiteness::Finite)
                && features.number == Some(Number::Singular)
                && matches!(features.person, Some(Person::First | Person::Third))
                && features.tense.is_none()
                && features.case.is_none()
        }
        // A finite form spelled alike in both numbers may leave number unmarked.
        WordForm::Present | WordForm::Preterite => {
            category == Category::Verb
                && features.finiteness == Some(Finiteness::Finite)
                && features.person.is_some()
                && features.case.is_none()
                && features.tense
                    == Some(if form == WordForm::Present { Tense::Present } else { Tense::Past })
        }
    }
}

/// The regular spelling of a Word Form of `lexeme`, from its lemma.
///
/// - Plural and third-person singular present: consonant + `y` becomes `ies`; a
///   final `s`, `x`, `z`, `ch` or `sh` takes `es`; otherwise `s`.
/// - Preterite and past participle: [`default_participle`].
/// - Gerund-participle: final `ie` becomes `ying`; a final `e` drops except
///   after `ee`, `oe` or `ye`; otherwise `ing`.
/// - Every other form is the lemma.
///
/// Consonant doubling and every other irregular spelling are explicit
/// overrides in the form declaration.
#[must_use]
pub fn default_surface(lexeme: &Lexeme, form: WordForm, features: &FeatureBundle) -> String {
    let lemma = &lexeme.lemma;
    match form {
        WordForm::Plural => suffix_s(lemma),
        WordForm::Present
            if features.person == Some(Person::Third)
                && features.number == Some(Number::Singular) =>
        {
            suffix_s(lemma)
        }
        WordForm::Preterite | WordForm::PastParticiple => past(lemma),
        WordForm::GerundParticiple => gerund(lemma),
        _ => lemma.clone(),
    }
}

fn consonant_y(stem: &str) -> Option<&str> {
    let base = stem.strip_suffix('y')?;
    let last = base.chars().next_back()?;
    (last.is_ascii_alphabetic() && !matches!(last, 'a' | 'e' | 'i' | 'o' | 'u')).then_some(base)
}

fn suffix_s(stem: &str) -> String {
    if let Some(base) = consonant_y(stem) {
        return format!("{base}ies");
    }
    if ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|ending| stem.ends_with(ending))
    {
        format!("{stem}es")
    } else {
        format!("{stem}s")
    }
}

/// The regular past-participle spelling, also used by explicitly declared
/// participial-adjective derivations: consonant + `y` becomes `ied`, a final
/// `e` takes `d`, otherwise `ed`. Irregular spellings replace this rule.
#[must_use]
pub fn default_participle(stem: &str) -> String {
    if let Some(base) = consonant_y(stem) {
        return format!("{base}ied");
    }
    if stem.ends_with('e') { format!("{stem}d") } else { format!("{stem}ed") }
}

fn past(stem: &str) -> String {
    default_participle(stem)
}

fn gerund(stem: &str) -> String {
    if let Some(base) = stem.strip_suffix("ie") {
        return format!("{base}ying");
    }
    if let Some(base) = stem.strip_suffix('e')
        && !["ee", "oe", "ye"]
            .iter()
            .any(|ending| stem.ends_with(ending))
    {
        return format!("{base}ing");
    }
    format!("{stem}ing")
}
