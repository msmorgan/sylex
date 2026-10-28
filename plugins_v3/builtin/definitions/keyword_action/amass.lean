/-- "<performer> amasses <subtype> N" with its reminder text (`Definition`, [CR#701.47a]): the
chosen Army is `that (.type .creature)` and `it`, resolved inside `ownScope`. -/
action amass (subtype : Subtype) (count : Amount) : Action := {
  -- TODO(spelling): amass <subtype> <count>; Oracle pluralizes the creature subtype, which string
  -- substitution cannot inflect.
  lexical := { verbs := [{ lemma := "amass", frames := [TransitiveAmount, Monotransitive] }] },
  body :=
    sequentially
      [doIf
          (not (exists_ Predicate.army))
          (create 1
            (written
              (creatureToken 0 0 [.black]
                [subtype, army]))),
        choose (a Predicate.army),
        putCounters count (printedKind p1p1Counter)
          (that (.type Semantics.CardType.creature)),
        doIf
          (not
            (is it (hasSubtype subtype)))
          (become it { characteristics := { subtypes := [subtype] } })]
}
