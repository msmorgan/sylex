/-- "that source": the source a choice or an event named, in any zone [CR#609.7a]. -/
-- [CR#609.7a]
template thatSource : NounPhrase := {
  spelling := "that source",
  body :=
    that Semantics.NounWord.source
}
