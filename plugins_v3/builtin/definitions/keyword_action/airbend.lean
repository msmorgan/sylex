/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
action airbend (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "airbend", past := "airbent", frames := [Monotransitive, Intransitive] }] },
  spelling := "airbend <subject>",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Definitions.exile subject,
        Semantics.Templates.Action.doForEach
          (Semantics.Templates.NounPhrase.allOf
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isCard,
                Semantics.Templates.Predicate.predicateDid
                  { subj := Semantics.Templates.Subject.any, action := Semantics.Definitions.exile Semantics.Templates.NounPhrase.which,
                    within := Option.some Semantics.Templates.LookbackSpan.thisWay }]))
          (Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.it)
            (Semantics.Templates.Action.establish
              (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.it))
                (Semantics.Definitions.cast Semantics.Templates.NounPhrase.it (Option.some Semantics.Templates.Location.exileZone)
                  (Semantics.Templates.PlayPayment.payingInstead (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 2]))))
              (Option.some
                (Semantics.Templates.Duration.forAsLongAs
                  (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.it
                    (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone))))))]
}
