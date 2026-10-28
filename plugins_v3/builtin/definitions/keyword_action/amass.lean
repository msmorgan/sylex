/-- "<performer> amasses <subtype> N" with its reminder text (`Definition`, [CR#701.47a]): the
chosen Army is `that (.type .creature)` and `it`, resolved inside `ownScope`. -/
action amass (subtype : Subtype) (count : Amount) : Action := {
  spelling := "amass <subtype> <count>",
  lexical := { verbs := [{ lemma := "amass", frames := [TransitiveAmount, Monotransitive] }] },
  body :=
    have isArmy : Predicate := and [hasSubtype army, isCreature, actorControls];
    sequentially
      [doIf
          (not (exists_ isArmy))
          (create 1
            (written
              (creatureToken 0 0 [black]
                [subtype, army]))),
        choose (a isArmy),
        putCounters count (printedKind p1p1Counter)
          (that (.type Semantics.CardType.creature)),
        doIf
          (not
            (is it (hasSubtype subtype)))
          (become it { characteristics := { subtypes := [subtype] } })]
}
