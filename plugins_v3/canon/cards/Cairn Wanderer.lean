card := {
  name := "Cairn Wanderer",
  cost := [4, black],
  types := [creature],
  subtypes := [shapeshifter],
  text := [
    changeling,
    alsoForKeywords
      (fun k => static
         (onlyWhile (keywordGrant thisCreature k [isCreature, in_ graveyard, hasKeyword k])
            (exists_ [isCreature, in_ graveyard, hasKeyword k])))
      [
        flying,
        fear,
        firstStrike,
        doubleStrike,
        deathtouch,
        haste,
        (anyIn ⟨landwalk, none⟩),
        lifelink,
        (anyIn ⟨protection, none⟩),
        reach,
        trample,
        shroud,
        vigilance
      ]
  ],
  power := 4,
  toughness := 4
}
