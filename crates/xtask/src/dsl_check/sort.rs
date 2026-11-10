//! Sorts, as written in headers and in the schema tables.

use std::collections::BTreeSet;
use std::fmt;

use sylex_syl::Sort;

use super::refusal::Refusal;
use super::refusal::RefusalKind;
use super::schema;

/// A sort: `Option(List(ManaSymbol))`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct SortKey {
    pub(super) head: String,
    pub(super) args: Vec<SortKey>,
}

impl SortKey {
    pub(super) fn named(head: &str) -> SortKey {
        SortKey {
            head: head.to_owned(),
            args: Vec::new(),
        }
    }

    /// Reads a schema table's sort, `Option(List(ManaSymbol))`.
    ///
    /// # Panics
    ///
    /// If the text is no sort: the tables are this crate's own.
    pub(super) fn table(text: &str) -> SortKey {
        let (key, rest) = SortKey::table_prefix(text);
        assert!(rest.is_empty(), "malformed schema sort `{text}`");
        key
    }

    fn table_prefix(text: &str) -> (SortKey, &str) {
        let end = text.find(['(', ')', ',']).unwrap_or(text.len());
        let mut key = SortKey::named(text[..end].trim());
        let mut rest = &text[end..];
        if let Some(inner) = rest.strip_prefix('(') {
            rest = inner;
            loop {
                let (arg, after) = SortKey::table_prefix(rest.trim_start());
                key.args.push(arg);
                if let Some(after) = after.strip_prefix(',') {
                    rest = after;
                } else {
                    rest = after.strip_prefix(')').expect("schema sort closes");
                    break;
                }
            }
        }
        (key, rest)
    }

    /// `U`, when this is `Option(U)`, where a `U` stands
    /// (`syl.md` §3).
    pub(super) fn option_inner(&self) -> Option<&SortKey> {
        (self.head == "Option").then(|| &self.args[0])
    }

    /// `U`, when this is `List(U)`, the sort a list literal stands at.
    pub(super) fn list_inner(&self) -> Option<&SortKey> {
        (self.head == "List").then(|| &self.args[0])
    }

    /// Every sort name in this sort, its arguments' included.
    pub(super) fn heads<'a>(&'a self, out: &mut BTreeSet<&'a str>) {
        out.insert(&self.head);
        for arg in &self.args {
            arg.heads(out);
        }
    }
}

impl fmt::Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.head)?;
        if !self.args.is_empty() {
            f.write_str("(")?;
            for (index, arg) in self.args.iter().enumerate() {
                if index > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{arg}")?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

/// A written sort, refusing a wrong number of sort arguments.
pub(super) fn sort_key(sort: &Sort) -> Result<SortKey, Refusal> {
    let head = sort.head.ident.as_str();
    let arity = usize::from(schema::SORT_CONSTRUCTORS.contains(&head));
    if sort.args.len() != arity {
        return Err(Refusal {
            span: sort.span,
            kind: RefusalKind::SortArity(head.to_owned(), arity),
        });
    }
    Ok(SortKey {
        head: head.to_owned(),
        args: sort.args.iter().map(sort_key).collect::<Result<_, _>>()?,
    })
}
