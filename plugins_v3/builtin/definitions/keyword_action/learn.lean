/-- "learn" (`Definition`, [CR#701.48a]): "You may discard a card. If you do, draw a card. If you
didn't discard a card, you may reveal a Lesson card you own from outside the game and put it into
your hand." The second condition is the discard's result, not the choice: a player who chose to
discard with no card in hand discarded none. -/
action learn : Action := {
  lexical := { verbs := [{ lemma := "learn", frames := [Intransitive] }] },
  spelling := "learn",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.may (Semantics.Definitions.discard (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCard))
          (Option.some (Semantics.Templates.Action.draw 1)),
        Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.exists_
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCard,
                  Semantics.Templates.Predicate.was (Semantics.Definitions.discard Semantics.Templates.NounPhrase.which)
                    (Option.some Semantics.Templates.LookbackSpan.thisWay)])))
          (Semantics.Templates.Action.may
            (Semantics.Templates.Action.sequentially
              [Semantics.Definitions.reveal
                  (Semantics.Templates.NounPhrase.a
                    (Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.lesson,
                        Semantics.Templates.Predicate.ownedBy Semantics.Templates.NounPhrase.actor, Semantics.Templates.Predicate.outsideTheGame])),
                Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.it
                  (Semantics.Templates.Location.handOf Semantics.Templates.NounPhrase.actor)]))]
}
