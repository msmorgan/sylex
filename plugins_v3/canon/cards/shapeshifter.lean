/-- Shapeshifter: its printed `*`/`7-*` box is filled by a Characteristic-Defining Ability
[CR#208.2a], one that reads the last chosen number. -/
card shapeshifter : Spelled :=
  { name := "Shapeshifter", cost := [generic 6], types := [.artifact, .creature],
    subtypes := [Definitions.shapeshifter],
    text :=
      [ static (entersChoosingFrom thisCreature .number (number (fromTo 0 7))),
        at_ (beginningOfYour .upkeep)
          (may (choose (a (qualityFrom .number (number (fromTo 0 7)))))),
        static (ptDefinitions thisCreature theLastChosenNumber
          (minus 7 theLastChosenNumber)) ] }
