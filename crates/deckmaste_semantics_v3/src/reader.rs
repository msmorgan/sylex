//! Balanced-delimiter reading of loose Lean declaration files.

use dpsi::Ident;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Ident(Ident),
    String(String),
    Atom(String),
    Group(char, Vec<Token>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Ident,
    pub implicit: bool,
    pub ty: Vec<Token>,
    pub default: Option<Vec<Token>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    pub command: Ident,
    pub name: Ident,
    pub return_type: Ident,
    pub parameters: Vec<Parameter>,
    pub fields: BTreeMap<Ident, Vec<Token>>,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ReadError(pub String);

struct Reader<'a> {
    source: &'a str,
    offset: usize,
}
impl Reader<'_> {
    fn skip(&mut self) -> Result<(), ReadError> {
        loop {
            let rest = &self.source[self.offset..];
            if rest.starts_with("--") {
                self.offset += rest.find('\n').unwrap_or(rest.len());
            } else if rest.starts_with("/-") {
                self.offset += 2;
                let mut depth = 1;
                while depth > 0 {
                    let rest = &self.source[self.offset..];
                    if rest.starts_with("/-") {
                        depth += 1;
                        self.offset += 2;
                    } else if rest.starts_with("-/") {
                        depth -= 1;
                        self.offset += 2;
                    } else if let Some(c) = rest.chars().next() {
                        self.offset += c.len_utf8();
                    } else {
                        return Err(ReadError("unterminated block comment".into()));
                    }
                }
            } else if let Some(c) = rest.chars().next().filter(|c| c.is_whitespace()) {
                self.offset += c.len_utf8();
            } else {
                return Ok(());
            }
        }
    }
    fn tokens(&mut self, end: Option<char>) -> Result<Vec<Token>, ReadError> {
        let mut tokens = Vec::new();
        loop {
            self.skip()?;
            let Some(c) = self.source[self.offset..].chars().next() else {
                return if end.is_some() {
                    Err(ReadError("unclosed delimiter".into()))
                } else {
                    Ok(tokens)
                };
            };
            self.offset += c.len_utf8();
            if Some(c) == end {
                return Ok(tokens);
            }
            if ")]}".contains(c) {
                return Err(ReadError(format!("unexpected closing delimiter {c}")));
            }
            if let Some(close) = match c {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                _ => None,
            } {
                tokens.push(Token::Group(c, self.tokens(Some(close))?));
            } else if c == '"' {
                let mut value = String::new();
                loop {
                    let Some(c) = self.source[self.offset..].chars().next() else {
                        return Err(ReadError("unterminated string".into()));
                    };
                    self.offset += c.len_utf8();
                    match c {
                        '"' => break,
                        '\\' => {
                            let Some(escaped) = self.source[self.offset..].chars().next() else {
                                return Err(ReadError("unterminated escape".into()));
                            };
                            self.offset += escaped.len_utf8();
                            value.push(match escaped {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                '"' => '"',
                                '\\' => '\\',
                                _ => {
                                    return Err(ReadError(format!(
                                        "unsupported string escape {escaped}"
                                    )));
                                }
                            });
                        }
                        _ => value.push(c),
                    }
                }
                tokens.push(Token::String(value));
            } else if c.is_alphabetic() || c == '_' || c == '«' {
                let start = self.offset - c.len_utf8();
                let mut quoted = c == '«';
                while let Some(next) = self.source[self.offset..].chars().next() {
                    if quoted {
                        self.offset += next.len_utf8();
                        if next == '»' {
                            quoted = false;
                        }
                    } else if next.is_alphanumeric() || "_'.".contains(next) {
                        self.offset += next.len_utf8();
                    } else if next == '«' {
                        quoted = true;
                        self.offset += next.len_utf8();
                    } else {
                        break;
                    }
                }
                if quoted {
                    return Err(ReadError("unterminated quoted identifier".into()));
                }
                tokens.push(Token::Ident(Ident::new(&self.source[start..self.offset])));
            } else if c.is_ascii_digit() {
                let start = self.offset - 1;
                while self.source[self.offset..]
                    .starts_with(|next: char| next.is_ascii_alphanumeric() || next == '_')
                {
                    self.offset += 1;
                }
                tokens.push(Token::Atom(self.source[start..self.offset].to_owned()));
            } else {
                let start = self.offset - c.len_utf8();
                let compound = [
                    ":=", "=>", "->", "<|", "|>", "++", "&&", "||", "==", "!=", "<=", ">=", "::",
                ]
                .into_iter()
                .find(|operator| self.source[start..].starts_with(operator));
                if let Some(operator) = compound {
                    self.offset = start + operator.len();
                }
                tokens.push(Token::Atom(self.source[start..self.offset].to_owned()));
            }
        }
    }
}

/// Tokenize Lean terms, preserving nested delimiters and ignoring comments.
///
/// # Errors
/// Returns malformed delimiter, comment, or string errors.
pub fn tokenize(source: &str) -> Result<Vec<Token>, ReadError> {
    Reader { source, offset: 0 }.tokens(None)
}

fn atom(token: &Token, value: &str) -> bool {
    matches!(token, Token::Atom(s) if s == value)
}
fn ident(token: &Token) -> Result<Ident, ReadError> {
    if let Token::Ident(id) = token {
        Ok(*id)
    } else {
        Err(ReadError("expected identifier".into()))
    }
}

/// Read a comma-separated declaration or catalog record.
///
/// # Errors
/// Rejects missing assignments, empty values, and duplicate fields.
pub fn fields(tokens: &[Token]) -> Result<BTreeMap<Ident, Vec<Token>>, ReadError> {
    let mut fields = BTreeMap::new();
    if tokens.is_empty() {
        return Ok(fields);
    }
    let tokens = if tokens.last().is_some_and(|t| atom(t, ",")) {
        &tokens[..tokens.len() - 1]
    } else {
        tokens
    };
    for part in tokens.split(|t| atom(t, ",")) {
        let [key, assignment, value @ ..] = part else {
            return Err(ReadError("expected field := value".into()));
        };
        let key = ident(key)?;
        if !atom(assignment, ":=") || value.is_empty() {
            return Err(ReadError(format!("expected value for {key}")));
        }
        if fields.insert(key, value.to_vec()).is_some() {
            return Err(ReadError(format!("duplicate field {key}")));
        }
    }
    Ok(fields)
}

fn result_type(command: &str) -> Option<&'static str> {
    match command {
        "action" => Some("Action"),
        "keyword" | "ability_word" => Some("Ability"),
        "counter" => Some("CounterKind"),
        "designation" => Some("DesignationLabel"),
        "subtype" => Some("Subtype"),
        "card_type" => Some("CardType"),
        "relation" => Some("RelationLabel"),
        "token" => Some("PredefinedToken"),
        "sba" => Some("SbaRule"),
        "conferral" => Some("ConferralRule"),
        "card" => Some("Spelled"),
        _ => None,
    }
}

/// Parse exactly one brace-and-comma declaration, independent of its file path.
///
/// # Errors
/// Rejects malformed signatures, duplicate fields, and extra declarations.
pub fn parse(source: &str) -> Result<Declaration, ReadError> {
    let tokens = tokenize(source)?;
    let [command, name, rest @ ..] = tokens.as_slice() else {
        return Err(ReadError("expected exactly one declaration".into()));
    };
    let command = ident(command)?;
    let mut name = ident(name)?;
    if result_type(command.as_str()).is_none()
        && !matches!(command.as_str(), "primitive" | "template")
    {
        return Err(ReadError(format!("unsupported command {command}")));
    }
    let mut rest = rest;
    let mut parameters = Vec::new();
    while let [Token::Group(open @ ('(' | '{'), binder), tail @ ..] = rest {
        let colon = binder
            .iter()
            .position(|t| atom(t, ":"))
            .ok_or_else(|| ReadError("parameter requires a type".into()))?;
        let value = &binder[colon + 1..];
        let assignment = value.iter().position(|t| atom(t, ":="));
        let ty = &value[..assignment.unwrap_or(value.len())];
        if ty.is_empty() {
            return Err(ReadError("empty parameter type".into()));
        }
        for name in &binder[..colon] {
            parameters.push(Parameter {
                name: ident(name)?,
                implicit: *open == '{',
                ty: ty.to_vec(),
                default: assignment.map(|i| value[i + 1..].to_vec()),
            });
        }
        rest = tail;
    }
    let explicit = if rest.first().is_some_and(|t| atom(t, ":")) {
        let end = rest
            .iter()
            .position(|t| atom(t, ":="))
            .ok_or_else(|| ReadError("expected declaration assignment".into()))?;
        if end <= 1 {
            return Err(ReadError("empty return type".into()));
        }
        let result = Ident::new(&render(&rest[1..end]));
        rest = &rest[end..];
        Some(result)
    } else {
        None
    };
    let result = result_type(command.as_str())
        .map(Ident::new)
        .or(explicit)
        .ok_or_else(|| ReadError("named return type required".into()))?;
    let return_type = Ident::new(
        result
            .as_str()
            .strip_prefix("Semantics.")
            .unwrap_or(result.as_str()),
    );
    if let Some(bare) = name.as_str().strip_prefix(&format!("{return_type}.")) {
        name = Ident::new(bare);
    }
    let [assignment, Token::Group('{', body)] = rest else {
        return Err(ReadError(
            "expected exactly one declaration with a braced body".into(),
        ));
    };
    if !atom(assignment, ":=") {
        return Err(ReadError("expected := before declaration body".into()));
    }
    let fields = if body.is_empty() { BTreeMap::new() } else { fields(body)? };
    let allowed: &[&str] = match command.as_str() {
        "primitive" | "template" => &["body"],
        "ability_word" => &["body", "label"],
        "keyword" => &["body", "label", "cost", "reminder"],
        "action" => &["body", "deed", "permanent", "ofAbility"],
        "counter" => &["holder", "confers", "rules"],
        "subtype" => &["kind", "name", "confers", "rules"],
        "designation" => &["scope", "sole", "confers", "rules"],
        "card_type" => &[
            "name",
            "permanent",
            "stats",
            "instantOrSorcery",
            "confers",
            "rules",
        ],
        "token" => &["name", "token", "body"],
        "sba" => &["scope", "when", "then_", "body"],
        "conferral" => &["scope", "confer", "body"],
        "card" => &[
            "name",
            "cost",
            "colors",
            "supertypes",
            "types",
            "subtypes",
            "text",
            "power",
            "toughness",
            "loyalty",
            "defense",
            "body",
        ],
        _ => &[],
    };
    for field in fields.keys() {
        if !matches!(field.as_str(), "lexical" | "spelling") && !allowed.contains(&field.as_str()) {
            return Err(ReadError(format!("unsupported {command} field {field}")));
        }
    }
    if let Some(value) = fields.get(&Ident::new("spelling"))
        && !matches!(value.as_slice(), [Token::String(_)])
    {
        return Err(ReadError("spelling must be a string".into()));
    }
    if let Some(value) = fields.get(&Ident::new("lexical")) {
        let [Token::Group('{', record)] = value.as_slice() else {
            return Err(ReadError("lexical must be a record".into()));
        };
        if !record.is_empty() {
            self::fields(record)?;
        }
    }
    Ok(Declaration {
        command,
        name,
        return_type,
        parameters,
        fields,
    })
}

/// Render tokenized terms with insignificant whitespace normalized.
#[must_use]
pub fn render(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Ident(id) => id.to_string(),
            Token::String(value) => format!("{value:?}"),
            Token::Atom(value) => value.clone(),
            Token::Group(open, inner) => format!(
                "{open}{}{close}",
                render(inner),
                close = match open {
                    '(' => ')',
                    '[' => ']',
                    _ => '}',
                }
            ),
        })
        .collect::<Vec<_>>()
        .join(" ")
}
