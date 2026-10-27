/-- Devour's definition over the permanents `quality` describes, counted by the variant's head
noun `head`: "As this object enters, you may sacrifice any number of [quality] permanents. This
permanent enters with N +1/+1 counters on it for each [head] sacrificed this way." Devour counts
each creature [CR#702.82a], devour [quality] each permanent [CR#702.82c]. -/
template devourEntry (quality head : Predicate) (amount : Amount) : Ability := {

  -- TODO(spelling): Devour entry wording inflects quality and head and repeats the counted action; plain substitution cannot select those forms.
  body :=
    Semantics.Templates.Ability.static
      (Semantics.Templates.StaticSpec.replace
        (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
        (Semantics.Templates.Replacement.before
          (Semantics.Templates.Action.may
            (Semantics.Definitions.Action.devour
              (Semantics.Templates.NounPhrase.counted Semantics.Templates.Quantity.anyNumber
                (Semantics.Templates.Predicate.and [quality, Semantics.Templates.Predicate.in_ Semantics.Templates.Location.battlefield]))))
          (Option.some
            (Semantics.Templates.Replacement.alongside
              (Semantics.Templates.Action.putCounters
                (Semantics.Templates.Amount.forEach amount
                  (Semantics.Templates.Predicate.and
                    [head,
                      Semantics.Templates.Predicate.was (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.which)
                        (Option.some Semantics.Templates.LookbackSpan.thisWay)]))
                (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter) Semantics.Templates.NounPhrase.thisPermanent)))))
}
