/-- As a spell is cast, its caster may reveal this card from their hand [CR#601.2b]; if they do,
`grants` applies to that spell, as with splice [CR#702.47a]. -/
primitive revealedAsCast (spell : NounPhrase) (grants : StaticSpec) : StaticSpec := {
  body := Semantics.Primitives.StaticSpec.revealedAsCast spell grants
}
