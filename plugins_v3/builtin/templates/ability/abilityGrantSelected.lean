/-- "<subject> has <selection> of <source>" (backup [CR#702.165a]). -/
-- [CR#702.165a]
template abilityGrantSelected (subject : NounPhrase) (source : NounPhrase)
    (selection : AbilitySelection) : StaticSpec := {
  spelling := "<subject> has <selection> of <source>",
  body :=
    characteristicChange subject [characteristicEditAddedSelectionOf source selection]
}
