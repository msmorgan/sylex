primitive revealedAsCast (spell : NounPhrase) (grants : StaticSpec) : StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.revealedAsCast spell grants
}
