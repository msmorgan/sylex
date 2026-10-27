/-- "an effect causes …", "if an effect would …": a resolving spell's or ability's effect, or a
replacement or prevention effect [CR#609.1,614.16]. Whether an Action is an effect follows from the
node that encloses it: a turn-based action is none [CR#703.1]. -/
-- [CR#609.1,614.16]
-- [CR#703.1]
primitive byAnEffect : Causing := {
  -- TODO(spelling): Context-dependent wording: an effect causes …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Causing.anEffect
}
