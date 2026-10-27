/-- "Suspend N—[cost]" represents three abilities [CR#702.62a]: "If you could begin to cast this
card by putting it onto the stack from your hand, you may pay [cost] and exile it with N time
counters on it. This action doesn't use the stack," a special action [CR#116.2f]; "At the
beginning of your upkeep, if this card is suspended, remove a time counter from it"; and "When
the last time counter is removed from this card, if it's exiled, you may play it without paying
its mana cost if able. If you don't, it remains exiled. If you cast a creature spell this way, it
gains haste until you lose control of the spell or the permanent it becomes." A suspended card is
in exile, has suspend and has a time counter on it [CR#702.62b]; whether the card could be cast
is the engine's to evaluate, prohibitions included [CR#702.62c]. Haste lasts for as long as you
control the spell or the permanent it becomes, which ends when you lose control of either. -/
keyword suspend (amount : Amount) (cost : Cost) : Ability := {
  lexical := {
    lemma := "suspend",
    parameters := [[Amount, DashCost]],
    verbs := [{ lemma := "suspend", frames := [Monotransitive] }]
  },
  spelling := "suspend <amount>—<cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.specialAction
          (Semantics.Templates.Timing.whileTrue
            (Semantics.Templates.Condition.couldBeginToCast Semantics.Templates.NounPhrase.you Semantics.Templates.NounPhrase.thisCard
              Semantics.Templates.Location.yourHand))
          cost
          (Semantics.Definitions.exileWithCounters Semantics.Templates.NounPhrase.thisCard amount Semantics.Definitions.timeCounter)),
      Semantics.Templates.Ability.at_ (Semantics.Templates.GameEvent.beginningOfYour Semantics.TurnPart.upkeep)
        (Semantics.Templates.Action.removeCounters (Semantics.Templates.Quantity.oneThrough 1)
          (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter)) Semantics.Templates.NounPhrase.thisCard)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisCard
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone,
                Semantics.Templates.Predicate.hasKeyword (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.suspend.label),
                Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.timeCounter)]))),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
          (Semantics.Templates.Action.removeCounters Semantics.Templates.Quantity.theLast
            (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter)) Semantics.Templates.NounPhrase.thisCard))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.may
              (Semantics.Definitions.play Semantics.Templates.NounPhrase.thisCard Option.none Semantics.Templates.PlayPayment.withoutPaying),
            Semantics.Templates.Action.doIf
              (Semantics.Templates.Condition.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
                (Semantics.Definitions.cast
                  (Semantics.Templates.NounPhrase.a
                    (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isSpell])))
                Semantics.Templates.LookbackSpan.thisWay)
              (Semantics.Templates.Action.gain (Semantics.Templates.NounPhrase.that Semantics.NounWord.spell) Semantics.Definitions.haste
                (Option.some
                  (Semantics.Templates.Duration.forAsLongAs
                    (Semantics.Templates.Condition.or
                      [Semantics.Templates.Condition.is (Semantics.Templates.NounPhrase.that Semantics.NounWord.spell)
                          Semantics.Templates.Predicate.youControl,
                        Semantics.Templates.Condition.is
                          (Semantics.Templates.NounPhrase.becomes (Semantics.Templates.NounPhrase.that Semantics.NounWord.spell))
                          Semantics.Templates.Predicate.youControl]))))])
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisCard (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone)))]
}
