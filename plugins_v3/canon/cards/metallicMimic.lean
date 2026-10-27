/-- Metallic Mimic. "This creature is the chosen type in addition to its other types" is a
Characteristic-Defining Ability: it depends on the choice made as it enters, as Shapeshifter's
power does, and depending on a choice is no condition [CR#604.3a]. Its counter on the other
creatures of the chosen type as they enter stays a separate ability, affecting other objects
[CR#604.3a]. -/
card metallicMimic : Spelled :=
  { name := "Metallic Mimic", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [shapeshifter],
    text :=
      [ static (entersChoosing thisCreature (.subtype .creature)),
        static (qualityDefinition thisCreature .adds (chosenQuality (ofChosen (.subtype
            .creature)))),
        static (entersWithAdditionalCounters
          (each (and [ isCreature, youControl, ofChosen (.subtype .creature),
                        isOther ]))
          1 p1p1Counter) ],
    power := stat 2, toughness := stat 1 }
