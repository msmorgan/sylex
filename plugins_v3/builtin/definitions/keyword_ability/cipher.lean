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
                (some .combat)))
            (may
              (sequentially
                [copy .fromCardZone
                    (the
                      (and
                        [isCard,
                          related encodedOn
                            (some thisCreature)]))
                    1 [],
                  may
                    (cast (that .copy) none
                      withoutPaying)]))))]
}
