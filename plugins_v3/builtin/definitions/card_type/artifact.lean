/-- Artifact: a permanent type [CR#110.4].

 [CR#301.1] An artifact card may be cast from its owner's hand as a sorcery. -/
card_type artifact := {
  lexical := { lemma := "artifact" },
  spelling := "artifact",
  name := "Artifact",
  permanent := true,
  stats := [],
  instantOrSorcery := false,
  confers := [castableFrom yourHand asSorcery]
}
