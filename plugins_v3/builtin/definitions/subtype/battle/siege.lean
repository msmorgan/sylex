/-- [CR#310.12a,310.12b,704.5v] A Siege's protector is an opponent of its controller, it is
exiled and may be cast transformed when its last defense counter is removed, and it is put into
its owner's graveyard at defense 0 once no ability of it waits to leave the stack. -/
subtype siege := {
  lexical := { lemma := "Siege" },
  spelling := "Siege",
  kind := .battle,
  name := "Siege",
  confers := [siegeProtector, siegeLastCounter, siegeDefenseZero]
}
