/-- "<performer> amasses <subtype> N" with its reminder text (`Definition`, [CR#701.47a]): the
chosen Army is `that (.type .creature)` and `it`, resolved inside `ownScope`. -/
action amass (subtype : Subtype) (count : Amount) : Action := {
  -- TODO(spelling): amass <subtype> <count>; Oracle pluralizes the creature subtype, which string
  -- substitution cannot inflect.
  lexical := { verbs := [{ lemma := "amass", frames := [TransitiveAmount, Monotransitive] }] },
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.not (Semantics.Templates.Condition.exists_ Semantics.Templates.Predicate.army))
          (Semantics.Definitions.create 1
            (Semantics.Templates.TokenSpec.written
              (Semantics.Templates.CharacteristicBundle.creatureToken 0 0 [Semantics.Color.black]
                [subtype, Semantics.Definitions.Subtype.army]))),
        Semantics.Templates.Action.choose (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.army),
        Semantics.Templates.Action.putCounters count (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          (Semantics.Templates.NounPhrase.that (Semantics.NounWord.type Semantics.CardType.creature)),
        Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.it (Semantics.Templates.Predicate.hasSubtype subtype)))
          (Semantics.Templates.Action.become Semantics.Templates.NounPhrase.it { characteristics := { subtypes := [subtype] } })]
}
