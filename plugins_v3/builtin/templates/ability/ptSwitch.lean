/-- "Switch <subject>'s power and toughness" [CR#613.4d] -/
-- [CR#613.4d]
template ptSwitch (subject : NounPhrase) : StaticSpec := {
  spelling := "Switch <subject>'s power and toughness",
  body :=
    characteristicChange subject [characteristicEditPtSwitch]
}
