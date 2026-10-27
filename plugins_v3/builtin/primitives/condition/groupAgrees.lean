/-- "Each <group> shares a card type", "… has a different name" [CR#702.139a] -/
-- [CR#702.139a]
primitive groupAgrees (group : NounPhrase) : DeckCondition := {
  spelling := "Each <group> shares a card type",
  body :=
    Semantics.Primitives.DeckCondition.groupAgrees group
}
