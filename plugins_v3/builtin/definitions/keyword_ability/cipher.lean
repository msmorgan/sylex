/-- "Cipher" means "If this spell is represented by a card, you may exile this card encoded on a
creature you control" and "For as long as this card is encoded on that creature, that creature
has 'Whenever this creature deals combat damage to a player, you may copy the encoded card and
you may cast the copy without paying its mana cost.'" [CR#702.99a] -/
keyword cipher : Ability := {
  lexical := { lemma := "cipher" },
  spelling := "cipher",
  body :=
    [spell
        (doIf (is this_ isCard)
          (may
            (sequentially
              [exile thisSpell,
                relate this_ encodedOn
                  (a
                    (and
                      [isCreature, youControl]))]))),
      static
        (abilityGrant
          (counterpart encodedOn this_)
          (whenever
            (does (actor thisCreature)
              (dealDamage anyAmount (a isPlayer)
                (some combat)))
            (may
              (sequentially
                [copy fromCardZone
                    (the
                      (and
                        [isCard,
                          related encodedOn
                            (some thisCreature)]))
                    1 [],
                  may
                    (cast (that copy) none
                      withoutPaying)]))))]
}
