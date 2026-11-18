//! Scope rules on `ownName` and `shortenedName`
//! (`docs/tickets/wip/syl-self-name-references.md`).
//!
//! Checks two invariants after names and sorts have resolved:
//! - neither `ownName` nor `shortenedName` (nor any word with `spelling` `"#"`
//!   or `"##"`) is called anywhere in the builtin bundle (save for
//!   `shortenedName`'s own `body = ownName` definition);
//! - `shortenedName` is refused on a card face for which
//!   `sylex_vocabulary::short_name(name, supertypes, types)` returns `None`.

use sylex_core::Supertype;
use sylex_core::TypeDef;
use sylex_spelling::OwnName;
use sylex_spelling::Spelling;
use sylex_syl::Call;
use sylex_syl::Item;
use sylex_syl::Record;
use sylex_syl::Shape;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use super::Decl;
use super::Refusal;
use super::RefusalKind;
use super::Registry;
use super::Resolutions;
use super::schema;
use super::sort::SortKey;

pub(super) fn check(
    decls: &[Decl<'_>],
    registry: &Registry,
    resolutions: &Resolutions,
    refusals: &mut Vec<Refusal>,
) {
    let checker = SelfNameChecker {
        decls,
        registry,
        resolutions,
    };
    for (decl_idx, decl) in decls.iter().enumerate() {
        checker.check_decl(decl_idx, decl, refusals);
    }
}

#[derive(Clone, Copy)]
struct FaceContext<'a> {
    face_name: Option<&'a str>,
    has_shortened_name: bool,
}

struct SelfNameChecker<'a, 'd> {
    decls: &'d [Decl<'a>],
    registry: &'d Registry,
    resolutions: &'d Resolutions,
}

impl<'a> SelfNameChecker<'a, '_> {
    /// Classifies `decl` if it is an own-name word (identified by `spelling`
    /// `"#"` / `"##"`, or `NounPhrase = OwnName`, or `shortenedName` at
    /// `NounPhrase`).
    fn own_name_form(decl: &Decl<'_>) -> Option<OwnName> {
        if let Some(spelling_val) = decl.field("spelling")
            && let Some(text) = as_string(spelling_val)
            && let Ok(Spelling::OwnName(form, _)) = sylex_spelling::parse(text)
        {
            return Some(form);
        }
        if !decl.sort.args.is_empty() || decl.sort.head != "NounPhrase" {
            return None;
        }
        if decl.kind.word == schema::PRIMITIVE
            && let Shape::Header(header) = &decl.declaration.shape
            && header
                .constructor
                .is_some_and(|ctor| ctor.ident.as_str() == "OwnName")
        {
            return Some(OwnName::Full);
        }
        if decl.kind.word == schema::TEMPLATE && decl.name.as_deref() == Some("shortenedName") {
            return Some(OwnName::Shortened);
        }
        None
    }

    /// Whether `caller_idx`'s entire body is a bare call to `OwnName::Full`
    /// defining a `Shortened` own-name Template (`shortenedName`'s own
    /// `body = ownName`).
    fn is_shortened_template_definition(&self, caller_idx: usize, call: &Call) -> bool {
        let caller = &self.decls[caller_idx];
        if caller.kind.word != schema::TEMPLATE
            || Self::own_name_form(caller) != Some(OwnName::Shortened)
        {
            return false;
        }
        let Some(body_call) = caller.field("body").and_then(as_call) else {
            return false;
        };
        std::ptr::eq(body_call, call) && call.is_bare()
    }

    fn check_decl(&self, decl_idx: usize, decl: &Decl<'a>, refusals: &mut Vec<Refusal>) {
        match &decl.declaration.shape {
            Shape::Value(value) => {
                if decl.in_builtin {
                    self.walk(decl_idx, true, value, None, refusals);
                } else if decl.kind.word == "card" {
                    self.check_card_value(decl_idx, value, refusals);
                } else {
                    self.walk(decl_idx, false, value, None, refusals);
                }
            }
            Shape::Relation(_) => {}
            Shape::Header(header) => {
                for param in header.params.iter().filter_map(Item::node) {
                    if let Some(default) = &param.default {
                        self.walk(decl_idx, decl.in_builtin, default, None, refusals);
                    }
                }
                if let Some(block) = &header.body {
                    for field in block.nodes() {
                        if !sylex_syl::METADATA_FIELDS.contains(&field.name.ident.as_str()) {
                            self.walk(decl_idx, decl.in_builtin, &field.value, None, refusals);
                        }
                    }
                }
            }
        }
    }

    fn check_card_value(&self, decl_idx: usize, value: &'a Value, refusals: &mut Vec<Refusal>) {
        let card_call = as_call(value).expect("resolved `card` value is a call");
        let call_sort = self
            .resolutions
            .call(card_call, None)
            .expect("resolved `card` call has a sort");
        let sig = self
            .registry
            .get(&(card_call.head.ident.to_string(), call_sort.clone()))
            .expect("resolved `card` call is in registry");

        let mut arg_by_param: Vec<Option<&'a Value>> = vec![None; sig.params.len()];
        for arg in card_call.args.iter().filter_map(Item::node) {
            let idx = self
                .resolutions
                .arg(arg)
                .expect("resolved `card` argument binds a parameter");
            arg_by_param[idx] = Some(&arg.value);
        }

        let characteristics_sort = SortKey::named("Characteristics");
        let shared_half_sort = SortKey::named("SharedLineHalf");
        let mut last_characteristics: Option<&'a Record> = None;
        let mut last_face: Option<FaceContext<'a>> = None;

        for (slot, arg_opt) in sig.params.iter().zip(arg_by_param) {
            let Some(arg_val) = arg_opt else {
                continue;
            };
            let ctx = if slot.sort == characteristics_sort {
                last_characteristics = as_record(arg_val);
                let c = self.face_context_from_characteristics(arg_val, None);
                last_face = Some(c);
                Some(c)
            } else if slot.sort == shared_half_sort {
                Some(self.face_context_from_characteristics(arg_val, last_characteristics))
            } else {
                last_face
            };
            self.walk(decl_idx, false, arg_val, ctx, refusals);
        }
    }

    fn face_context_from_characteristics(
        &self,
        face_val: &'a Value,
        shared_type_line: Option<&'a Record>,
    ) -> FaceContext<'a> {
        let rec =
            as_record(face_val).expect("resolved `Characteristics` / `SharedLineHalf` is a record");
        let face_name = rec
            .nodes()
            .find(|f| f.name.ident == "name")
            .and_then(|f| as_string(&f.value));
        let type_source = shared_type_line.unwrap_or(rec);
        let supertypes = self.extract_supertypes(type_source);
        let types = self.extract_types(type_source);
        let has_shortened_name = face_name
            .and_then(|name| sylex_vocabulary::short_name(name, &supertypes, &types))
            .is_some();
        FaceContext {
            face_name,
            has_shortened_name,
        }
    }

    fn extract_supertypes(&self, rec: &Record) -> Vec<Supertype> {
        let Some(field) = rec.nodes().find(|f| f.name.ident == "supertypes") else {
            return Vec::new();
        };
        let items = as_list(&field.value).expect("resolved `supertypes` field is a list");
        let supertype_sort = SortKey::named("Supertype");
        let mut out = Vec::new();
        for item in items.iter().filter_map(Item::node) {
            let call = as_call(item).expect("resolved `Supertype` item is a call");
            let sig = self
                .registry
                .get(&(call.head.ident.to_string(), supertype_sort.clone()))
                .expect("resolved `Supertype` call is in registry");
            let sdecl = &self.decls[sig.decl];
            let canonical = sdecl.field("name").map_or(call.head.ident.as_str(), |v| {
                as_string(v).expect("`supertype.name` is a string")
            });
            let st = Supertype::from_name(canonical)
                .unwrap_or_else(|| panic!("unknown supertype `{canonical}`"));
            out.push(st);
        }
        out
    }

    fn extract_types(&self, rec: &Record) -> Vec<TypeDef> {
        let Some(field) = rec.nodes().find(|f| f.name.ident == "types") else {
            return Vec::new();
        };
        let items = as_list(&field.value).expect("resolved `types` field is a list");
        let card_type_sort = SortKey::named("CardType");
        let bool_sort = SortKey::named("Bool");
        let mut out = Vec::new();
        for item in items.iter().filter_map(Item::node) {
            let call = as_call(item).expect("resolved `CardType` item is a call");
            let sig = self
                .registry
                .get(&(call.head.ident.to_string(), card_type_sort.clone()))
                .expect("resolved `CardType` call is in registry");
            let tdecl = &self.decls[sig.decl];
            let canonical = tdecl.field("name").map_or(call.head.ident.as_str(), |v| {
                as_string(v).expect("`card_type.name` is a string")
            });
            let bcall = tdecl
                .field("permanentType")
                .and_then(as_call)
                .expect("resolved `card_type.permanentType` is a Bool call");
            let bsig = self
                .registry
                .get(&(bcall.head.ident.to_string(), bool_sort.clone()))
                .expect("resolved `Bool` call is in registry");
            let permanent_type = match &self.decls[bsig.decl].declaration.shape {
                Shape::Header(h) => h.constructor.is_some_and(|c| c.ident == "True"),
                Shape::Value(_) | Shape::Relation(_) => false,
            };
            // `sylex_vocabulary::short_name` reads only `name` and
            // `permanent_type`.
            out.push(TypeDef {
                name: canonical.into(),
                permanent_type,
                confers: [].into(),
            });
        }
        out
    }

    fn walk(
        &self,
        caller_idx: usize,
        in_builtin: bool,
        value: &'a Value,
        face: Option<FaceContext<'a>>,
        refusals: &mut Vec<Refusal>,
    ) {
        match &value.kind {
            ValueKind::List(items) => {
                for item in items.iter().filter_map(Item::node) {
                    self.walk(caller_idx, in_builtin, item, face, refusals);
                }
            }
            ValueKind::Record(rec) => {
                for field in rec.nodes() {
                    if !sylex_syl::METADATA_FIELDS.contains(&field.name.ident.as_str()) {
                        self.walk(caller_idx, in_builtin, &field.value, face, refusals);
                    }
                }
            }
            ValueKind::Call(call) => {
                // No declaration with an `at` list names a card, so `Self`
                // names no sort here and such a call is no own-name word.
                if let Some(sort) = self.resolutions.call(call, None)
                    && let Some(sig) = self
                        .registry
                        .get(&(call.head.ident.to_string(), sort.clone()))
                    && !sig.secondary
                    && let Some(form) = Self::own_name_form(&self.decls[sig.decl])
                {
                    let word = call.head.ident.as_str();
                    if in_builtin {
                        if !self.is_shortened_template_definition(caller_idx, call) {
                            refusals.push(Refusal {
                                span: call.span,
                                kind: RefusalKind::OwnNameInBuiltin(word.to_owned()),
                            });
                        }
                    } else if form == OwnName::Shortened {
                        let has_short = face.is_some_and(|f| f.has_shortened_name);
                        if !has_short {
                            let face_label = face
                                .and_then(|f| f.face_name)
                                .unwrap_or("<unnamed>")
                                .to_owned();
                            refusals.push(Refusal {
                                span: call.span,
                                kind: RefusalKind::ShortenedNameWithoutShortening {
                                    word: word.to_owned(),
                                    face: face_label,
                                },
                            });
                        }
                    }
                }
                for arg in call.args.iter().filter_map(Item::node) {
                    self.walk(caller_idx, in_builtin, &arg.value, face, refusals);
                }
            }
            ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::Parameter(_) => {}
        }
    }
}

fn as_record(value: &Value) -> Option<&Record> {
    match &value.kind {
        ValueKind::Record(rec) => Some(rec),
        ValueKind::List(_)
        | ValueKind::Call(_)
        | ValueKind::String(_)
        | ValueKind::Integer(_)
        | ValueKind::Parameter(_) => None,
    }
}

fn as_call(value: &Value) -> Option<&Call> {
    match &value.kind {
        ValueKind::Call(call) => Some(call),
        ValueKind::Record(_)
        | ValueKind::List(_)
        | ValueKind::String(_)
        | ValueKind::Integer(_)
        | ValueKind::Parameter(_) => None,
    }
}

fn as_string(value: &Value) -> Option<&str> {
    match &value.kind {
        ValueKind::String(s) => Some(s.as_str()),
        ValueKind::Record(_)
        | ValueKind::List(_)
        | ValueKind::Call(_)
        | ValueKind::Integer(_)
        | ValueKind::Parameter(_) => None,
    }
}

fn as_list(value: &Value) -> Option<&[Item<Value>]> {
    match &value.kind {
        ValueKind::List(items) => Some(items.as_slice()),
        ValueKind::Record(_)
        | ValueKind::Call(_)
        | ValueKind::String(_)
        | ValueKind::Integer(_)
        | ValueKind::Parameter(_) => None,
    }
}
