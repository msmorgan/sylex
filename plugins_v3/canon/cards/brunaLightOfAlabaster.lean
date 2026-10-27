/-- Bruna, Light of Alabaster: Auras already on the battlefield are attached [CR#701.3a]; Aura
cards are put onto the battlefield attached to it, each one that could enchant it
[CR#303.4i]. -/
card brunaLightOfAlabaster : Spelled :=
  { name := "Bruna, Light of Alabaster", cost := [generic 3, pip .white, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [angel],
    text :=
      [ flying, vigilance,
        whenever (does thisCreature attack) (or_ := [does thisCreature block])
          (sequentially
            [ may (attachTo (counted anyNumber (and [hasSubtype aura, in_ battlefield])) it),
              may (putOntoBattlefieldAttachedTo
                (counted anyNumber
                  (and [hasSubtype aura, cardIn (andOr [yourGraveyard, yourHand]),
                    couldEnchant it]))
                it) ]) ],
    power := stat 5, toughness := stat 5 }
