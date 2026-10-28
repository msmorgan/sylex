/-- The new object the subject becomes at its next zone change, named before the change happens:
"the permanent this spell becomes" [CR#400.7]. -/
primitive becomes (subject : NounPhrase) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.becomes subject
}
