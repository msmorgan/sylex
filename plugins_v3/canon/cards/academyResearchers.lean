/-- Academy Researchers: the Aura enters attached to the creature the effect names, so enchant's
own choice of host doesn't apply; one that can't legally enchant it stays in its owner's hand
[CR#303.4f,303.4i]. -/
card academyResearchers : Spelled :=
  { name := "Academy Researchers", cost := [generic 1, pip .blue, pip .blue],
    types := [.creature], subtypes := [human, wizard],
    text :=
      [ when (does thisCreature enter)
          (may (putOntoBattlefieldAttachedTo (a (and [hasSubtype aura, cardIn yourHand]))
            thisCreature)) ],
    power := stat 2, toughness := stat 2 }
