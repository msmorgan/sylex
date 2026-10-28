/-- The number of defense counters on a battle on the battlefield is its defense. A battle with zero
defense is put into its owner's graveyard if it is not the source of an ability that has
triggered but not yet left the stack. [CR#122.1g] -/
counter defenseCounter := {
  lexical := { lemma := "defense counter" },
  spelling := "defense counter",
  holder := object,
  confers := []
}
