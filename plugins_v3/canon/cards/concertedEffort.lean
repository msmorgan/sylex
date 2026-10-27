/-- Concerted Effort -/
card concertedEffort : Spelled :=
  { name := "Concerted Effort", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ alsoForKeywords
          (fun k => at_ (beginningOfPossessed .upkeep eachPlayer)
            (doOnlyIf
              (establishThroughout
                (keywordGrant (allOf (and [isCreature, youControl])) k
                  (and [isCreature, youControl, hasKeyword k]))
                untilEndOfTurn)
              (exists_ (and [isCreature, youControl, hasKeyword k]))))
          [ flying, fear, firstStrike, doubleStrike, (.anyIn ⟨landwalk, none⟩), (.anyIn ⟨protection,
            none⟩),
            trample, vigilance ] ] }
