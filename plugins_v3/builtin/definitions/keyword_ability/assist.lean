/-- "Assist" [CR#702.132a]: while you cast this spell, you may choose another player, and before
you begin to pay its total cost the chosen player may pay for any amount of its generic mana. The
choice is no cost, so it is made as you cast the spell, not added to its costs; where it falls
among the steps of casting [CR#601.2g] is the engine's. The generic mana the chosen player pays
is a Mana Substitute, as convoke's tapped creatures are [CR#702.51a]. -/
keyword assist : Ability := {
  lexical := { lemma := "assist" },
  spelling := "assist",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.asEvent
          (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
            (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
          (Semantics.Templates.Action.may
            (Semantics.Templates.Action.choose
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.isOther]))))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.paySubstitute Semantics.Templates.NounPhrase.thisSpell
          {
            action :=
              Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.the Semantics.Templates.Predicate.isChosenPlayer)
                (Semantics.Templates.Action.pay (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 1])
                  Semantics.PayTimes.once),
            pays := [Semantics.Templates.PipPaid.generic] })]
}
