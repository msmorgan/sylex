/-- Trample is a static ability that modifies the rules for assigning an attacking creature’s combat
damage. [CR#702.19a] -/
keyword trample : Ability := {
  lexical := { lemma := "trample", parameters := [[], [Quality (Some ("over"))]] },
  spelling := "trample",
  body :=
    []
}
