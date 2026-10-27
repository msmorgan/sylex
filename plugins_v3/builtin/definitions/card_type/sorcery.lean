/-- Sorcery: not a permanent type [CR#110.4]; a sorcery card is an instant or sorcery card
[CR#110.4a,307.1].

 [CR#307.1] A sorcery card may be cast from its owner's hand as a sorcery. -/
card_type sorcery := {
  lexical := { lemma := "sorcery" },
  spelling := "sorcery",
  name := "Sorcery",
  permanent := false,
  instantOrSorcery := true,
  stats := [],
  confers := [castableFrom yourHand asSorcery]
}
