keyword rebound : Ability := {
  lexical := { lemma := "rebound" },
  spelling := "rebound",
  body :=
    [ static
        (replace
          (.causedBy (.event (does .any (resolve thisSpell)))
            (.would .any (putInto thisSpell graveyard (from_ := stack))))
          (doIf (is thisSpell (did any (cast which (from_ := yourHand)) thisObject))
            (instead
              (sequentially
                [ exile thisSpell,
                  delay (beginningOf .upkeep (some .you) (.next none))
                    (may (cast this_ (payment := withoutPaying))) ])))) ]
}
