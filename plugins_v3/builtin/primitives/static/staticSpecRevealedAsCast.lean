primitive staticSpecRevealedAsCast (spell : Semantics.NounPhrase) (grants : Semantics.StaticSpec) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.revealedAsCast spell grants
}
