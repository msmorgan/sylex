/-- As a spell is cast, its caster may reveal this card from their hand [CR#601.2b]; if they do,
`grants` applies to that spell, as with splice [CR#702.47a]. -/
primitive revealedAsCast (spell : NounPhrase) (grants : StaticSpec) : StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.revealedAsCast spell grants
}
