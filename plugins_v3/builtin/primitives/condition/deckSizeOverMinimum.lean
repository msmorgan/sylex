/-- "at least <extra> cards more than the minimum deck size" -/
primitive deckSizeOverMinimum (extra : Nat) : DeckCondition := {
  spelling := "at least <extra> cards more than the minimum deck size",
  body :=
    Semantics.Primitives.DeckCondition.deckSizeOverMinimum extra
}
