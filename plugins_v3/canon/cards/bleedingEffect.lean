card bleedingEffect : Spelled :=
  { name := "Bleeding Effect", cost := [generic 2, pip .white, pip .black],
    types := [.enchantment],
    text :=
      [ alsoForKeywords
          (fun k => at_ (beginningOfYour .combat)
            (doOnlyIf
              (establishThroughout
                (keywordGrant (allOf (and [isCreature, youControl])) k
                  (and [isCreature, in_ yourGraveyard, hasKeyword k]))
                untilEndOfTurn)
              (exists_ (and [isCreature, in_ yourGraveyard, hasKeyword k]))))
          [ flying, firstStrike, doubleStrike, deathtouch, (.anyIn ⟨hexproof, none⟩),
            indestructible,
            lifelink, menace, reach, trample, vigilance ] ] }
