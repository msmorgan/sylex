/-- "Frenzy N": "Whenever this creature attacks and isn't blocked, it gets +N/+0 until end of
turn." [CR#702.68a] -/
keyword frenzy (amount : Amount) : Ability := {
  lexical := { lemma := "frenzy", parameters := [[Amount]] },
  spelling := "frenzy <amount>",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.actionAttackUnblocked Option.none))
        (Semantics.Templates.Action.get Semantics.Templates.NounPhrase.thisCreature (Semantics.Templates.up amount) (Semantics.Templates.up 0)
          (Option.some Semantics.Templates.Duration.untilEndOfTurn))]
}
