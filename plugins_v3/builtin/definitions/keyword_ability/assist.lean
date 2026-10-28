/-- "Assist" [CR#702.132a]: while you cast this spell, you may choose another player, and before
you begin to pay its total cost the chosen player may pay for any amount of its generic mana. The
choice is no cost, so it is made as you cast the spell, not added to its costs; where it falls
among the steps of casting [CR#601.2g] is the engine's. The generic mana the chosen player pays
is a Mana Substitute, as convoke's tapped creatures are [CR#702.51a]. -/
keyword assist : Ability := {
  lexical := { lemma := "assist" },
  spelling := "assist",
  body :=
    [static
        (asEvent
          (does (actor you)
            (cast thisSpell))
          (may
            (choose
              (a
                (and [isPlayer, isOther]))))),
      static
        (paySubstitute thisSpell
          {
            action :=
              instruct (the isChosenPlayer)
                (pay (mana [generic 1])
                  once),
            pays := [generic] })]
}
