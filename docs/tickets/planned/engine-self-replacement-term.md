---
needs: []
---

# Engine: "self-replacement" names only [CR#614.15] self-replacement effects

## Why

A self-replacement effect is an effect of a resolving spell or ability that replaces part of
that spell or ability's own effect, and it is chosen before every other replacement
[CR#614.15,616.1a]. `ReplacementTier::SelfReplacement` (`crates/engine/src/replace_registry.rs`)
carries that meaning. Elsewhere the engine's comments, messages and test names use
"self-replacement" for a permanent's or card's static replacement that applies to itself:
madness, the "as this enters" and "enters tapped" replacements, enters-attached Auras. Those are
continuous effects of static abilities [CR#611.3,614.12], in the [CR#616.1e] tier. The misnomer
invites tagging one of them with the wrong tier.

## Done when

- No comment, message or identifier in `crates/engine` calls a static ability's replacement a
  self-replacement; "self-replacement" appears only for [CR#614.15] effects.

## Pointers

- `crates/engine/src/{copy,derive,event,replace,replace_registry}.rs`,
  `crates/engine/src/step/mod.rs`, `crates/engine/src/resolve/action.rs`
