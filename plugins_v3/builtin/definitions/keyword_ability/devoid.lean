/-- "Devoid" means "This object is colorless": a Characteristic-Defining Ability
[CR#702.114a,604.3a]. -/
keyword devoid : Ability := {
  lexical := { lemma := "devoid" },
  spelling := "devoid",
  body :=
    [static (definesQuality this_ sets (colored (.some [])))]
}
