/-- Concerted Effort -/
card := {
  name := "Concerted Effort",
  cost := [2, white, white],
  types := [enchantment],
  text := [
    alsoForKeywords
      (fun k => at_ (beginningOfPossessed upkeep eachPlayer)
         (doOnlyIf
            (establishThroughout
               (keywordGrant (allOf [isCreature, youControl]) k
                  [isCreature, youControl, hasKeyword k])
               untilEndOfTurn)
            (exists_ [isCreature, youControl, hasKeyword k])))
      [
        flying,
        fear,
        firstStrike,
        doubleStrike,
        (anyIn ⟨landwalk, none⟩),
        (anyIn ⟨protection, none⟩),
        trample,
        vigilance
      ]
  ]
}
