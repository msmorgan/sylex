/-- A spell cast as an Adventure is exiled as it resolves instead of going to its owner's graveyard;
its controller may play the card from exile, but not as an Adventure through that permission.
[CR#715.3d]

No supported Oracle rules-text or Comprehensive Rules plural is attested. -/
subtype adventure := {
  lexical := { lemma := "Adventure" },
  spelling := "Adventure",
  kind := .spell,
  name := "Adventure",
  confers := []
}
