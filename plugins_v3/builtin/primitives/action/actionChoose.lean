primitive actionChoose (first : Option Semantics.NounPhrase) (chosen : Semantics.NounPhrase) (disclosure : Semantics.Disclosure) (when : Option Semantics.Timing) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.choose first chosen disclosure when
}
