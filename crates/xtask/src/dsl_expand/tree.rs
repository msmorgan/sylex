//! The expanded tree: plain data, compared with `==` and printed for people.

use std::fmt;
use std::fmt::Write as _;

/// One node of an expanded term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// A primitive's constructor, with an argument for every parameter in
    /// declaration order, defaults filled in.
    Call {
        /// The constructor its primitive's constructor clause names (`syl.md`
        /// §1).
        constructor: String,
        /// The primitive's word.
        word: String,
        sort: String,
        args: Vec<Node>,
    },
    /// A word that names a declaration and calls nothing: a catalog
    /// declaration, or a declaration at its kind's second sort.
    Word {
        word: String,
        sort: String,
    },
    /// Where a Definition was entered.
    Mark {
        definition: String,
        sort: String,
        deed: Option<Deed>,
        body: Box<Node>,
    },
    /// A parameter left free, so an opaque value: a test's `X`, or a
    /// declaration's own parameter when its body is expanded alone.
    Param(String),
    String(String),
    Integer(i64),
    List(Vec<Node>),
    /// The fields in name order.
    Record(Vec<(String, Node)>),
    /// `none` at an `Option` position.
    None,
}

/// The Deed a Definition's declaration states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Deed {
    /// A Deed label: the declaration's `deed` string, or its own name when it
    /// writes none.
    Label(String),
    /// The Deed of the keyword ability its `ofAbility` names.
    OfAbility(String),
}

impl Node {
    /// Every node of the tree, this one first, depth first.
    #[must_use]
    pub fn descendants(&self) -> Vec<&Node> {
        let mut out = vec![self];
        let mut index = 0;
        while index < out.len() {
            let node = out[index];
            match node {
                Node::Call { args, .. } => out.extend(args),
                Node::Mark { body, .. } => out.push(body),
                Node::List(items) => out.extend(items),
                Node::Record(fields) => out.extend(fields.iter().map(|(_, node)| node)),
                Node::Word { .. }
                | Node::Param(_)
                | Node::String(_)
                | Node::Integer(_)
                | Node::None => {}
            }
            index += 1;
        }
        out
    }

    /// This tree with every node `matches` holds of replaced by `to`.
    #[must_use]
    pub fn replace(&self, matches: &dyn Fn(&Node) -> bool, to: &Node) -> Node {
        if matches(self) {
            return to.clone();
        }
        let each = |nodes: &[Node]| nodes.iter().map(|n| n.replace(matches, to)).collect();
        match self {
            Node::Call {
                constructor,
                word,
                sort,
                args,
            } => Node::Call {
                constructor: constructor.clone(),
                word: word.clone(),
                sort: sort.clone(),
                args: each(args),
            },
            Node::Mark {
                definition,
                sort,
                deed,
                body,
            } => Node::Mark {
                definition: definition.clone(),
                sort: sort.clone(),
                deed: deed.clone(),
                body: Box::new(body.replace(matches, to)),
            },
            Node::List(items) => Node::List(each(items)),
            Node::Record(fields) => Node::Record(
                fields
                    .iter()
                    .map(|(name, node)| (name.clone(), node.replace(matches, to)))
                    .collect(),
            ),
            Node::Word { .. }
            | Node::Param(_)
            | Node::String(_)
            | Node::Integer(_)
            | Node::None => self.clone(),
        }
    }

    /// The tree on one line.
    fn flat(&self) -> String {
        let mut out = String::new();
        self.write_flat(&mut out);
        out
    }

    fn write_flat(&self, out: &mut String) {
        let join = |out: &mut String, nodes: &[Node]| {
            for (index, node) in nodes.iter().enumerate() {
                if index > 0 {
                    out.push_str(", ");
                }
                node.write_flat(out);
            }
        };
        match self {
            Node::Call {
                constructor, args, ..
            } => {
                out.push_str(constructor);
                if !args.is_empty() {
                    out.push('(');
                    join(out, args);
                    out.push(')');
                }
            }
            Node::Word { word, .. } => out.push_str(word),
            Node::Mark { body, .. } => {
                out.push_str(&self.mark_head());
                out.push_str(" { ");
                body.write_flat(out);
                out.push_str(" }");
            }
            Node::Param(name) => {
                out.push('$');
                out.push_str(name);
            }
            Node::String(text) => write!(out, "{text:?}").expect("writing to a String"),
            Node::Integer(number) => write!(out, "{number}").expect("writing to a String"),
            Node::List(items) => {
                out.push('[');
                join(out, items);
                out.push(']');
            }
            Node::Record(fields) => {
                out.push('{');
                for (index, (name, node)) in fields.iter().enumerate() {
                    if index > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(name);
                    out.push_str(" = ");
                    node.write_flat(out);
                }
                out.push('}');
            }
            Node::None => out.push_str("none"),
        }
    }

    /// `mark destroy: Action, deed "destroy"`.
    fn mark_head(&self) -> String {
        let Node::Mark {
            definition,
            sort,
            deed,
            ..
        } = self
        else {
            unreachable!("only a mark has a mark head");
        };
        let deed = match deed {
            Some(Deed::Label(label)) => format!("deed {label:?}"),
            Some(Deed::OfAbility(label)) => format!("deed of ability {label}"),
            None => "no deed".to_owned(),
        };
        format!("mark {definition}: {sort}, {deed}")
    }

    /// Writes the tree at `indent`, breaking a node whose one-line form would
    /// pass `WIDTH` into one child per line.
    fn write(&self, indent: usize, out: &mut String) {
        const WIDTH: usize = 100;
        let flat = self.flat();
        if indent + flat.len() <= WIDTH {
            out.push_str(&flat);
            return;
        }
        let pad = " ".repeat(indent + 2);
        let children = |out: &mut String, nodes: &[(Option<&str>, &Node)], open, close| {
            out.push_str(open);
            out.push('\n');
            for (name, node) in nodes {
                out.push_str(&pad);
                if let Some(name) = name {
                    out.push_str(name);
                    out.push_str(" = ");
                }
                node.write(indent + 2, out);
                out.push_str(",\n");
            }
            out.push_str(&" ".repeat(indent));
            out.push_str(close);
        };
        match self {
            Node::Call {
                constructor, args, ..
            } => {
                out.push_str(constructor);
                let args: Vec<_> = args.iter().map(|node| (None, node)).collect();
                children(out, &args, "(", ")");
            }
            Node::Mark { body, .. } => {
                out.push_str(&self.mark_head());
                out.push(' ');
                children(out, &[(None, body)], "{", "}");
            }
            Node::List(items) => {
                let items: Vec<_> = items.iter().map(|node| (None, node)).collect();
                children(out, &items, "[", "]");
            }
            Node::Record(fields) => {
                let fields: Vec<_> = fields
                    .iter()
                    .map(|(name, node)| (Some(name.as_str()), node))
                    .collect();
                children(out, &fields, "{", "}");
            }
            Node::Word { .. }
            | Node::Param(_)
            | Node::String(_)
            | Node::Integer(_)
            | Node::None => out.push_str(&flat),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write(0, &mut out);
        f.write_str(&out)
    }
}
