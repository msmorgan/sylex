/-- "Prowl <cost>": cast for <cost> if a player was dealt combat damage this turn by a source
you controlled that shared a creature type with this spell [CR#702.76a]. -/
keyword prowl (cost : Cost) : Ability := {
  lexical := { lemma := "prowl", parameters := [[Cost]] },
  spelling := "prowl <cost>",
  body :=
    [static
        (onlyWhile (altCost this_ (some cost))
          (did
            (actor
              (a
                (and
                  [shares (Semantics.QualitySort.subtype Semantics.CardType.creature)
                      Semantics.Agreement.share thisSpell,
                    youControl])))
            (dealDamage anyAmount (a isPlayer)
              (some Semantics.DamageKind.combat))
            (within soFarThisTurn)))]
}
