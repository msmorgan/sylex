/-- "Replace all instances of one <sort> word with another", the kind one of `sorts` [CR#612.2];
an absent word is the performer's choice. -/
-- [CR#612.2]
primitive replaceWord (sorts : List TextWordSort) (from_ : Option TextWord := none)
    (to : Option TextWord := none) : TextChange := {
  -- TODO(spelling): Context-dependent wording: Replace all instances of one <sort> word with another; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.TextChange.replaceWord sorts from_ to
}
