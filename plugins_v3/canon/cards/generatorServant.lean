/-- Generator Servant -/
card generatorServant : Spelled :=
  { name := "Generator Servant", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ activated (compound [tapSymbol, sacrifice thisCreature])
          (addMana 1 (manaOf [[.colorless, .colorless]])
            [ whenSpentOn .affectsIt false (a (and [isCreature, isSpell]))
                (gainHaste (becomes (that .spell)) untilEndOfTurn) ]) ],
    power := stat 2, toughness := stat 1 }
