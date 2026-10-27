/-- Cairn Wanderer -/
card cairnWanderer : Spelled :=
  { name := "Cairn Wanderer", cost := [generic 4, pip .black], types := [.creature],
    subtypes := [shapeshifter],
    text :=
      [ changeling,
        alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k (and [isCreature, in_ graveyard, hasKeyword k]))
            (exists_ (and [isCreature, in_ graveyard, hasKeyword k]))))
          [ flying, fear, firstStrike, doubleStrike, deathtouch, haste,
            (.anyIn ⟨landwalk, none⟩), lifelink, (.anyIn ⟨protection, none⟩), reach, trample,
            shroud, vigilance ] ],
    power := stat 4, toughness := stat 4 }
