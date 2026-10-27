primitive staticSpecConjunction (subject : Option Semantics.NounPhrase) (parts : List Semantics.StaticSpec) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.conjunction subject parts
}
