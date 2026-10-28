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
                (some Semantics.DamageKind.combat)))
            (may
              (sequentially
                [copy Semantics.CopySort.fromCardZone
                    (the
                      (and
                        [isCard,
                          related encodedOn
                            (some thisCreature)]))
                    1 [],
                  may
                    (cast (that Semantics.NounWord.copy) none
                      withoutPaying)]))))]
}
