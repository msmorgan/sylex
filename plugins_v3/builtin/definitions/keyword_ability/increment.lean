keyword increment : Ability := {
  lexical := { lemma := "increment" },
  spelling := "increment",
  body :=
    [ whenever (does you (cast (a isSpell)))
        (if_ := some (and
          [ is thisPermanent isCreature,
            or
              [ .compareAmt (manaValueSpentToCast (that .spell)) .greater (powerOf thisCreature),
                .compareAmt (manaValueSpentToCast (that .spell)) .greater
                  (toughnessOf thisCreature) ] ]))
        (putCounters 1 p1p1Counter thisCreature) ]
}
