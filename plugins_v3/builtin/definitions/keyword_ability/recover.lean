keyword recover (cost : Cost) : Ability := {
  lexical := { lemma := "recover", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): recover [mana cost] / recover—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [when
        (does any
          (move (a isCreature) battlefield
            (some yourGraveyard) .departure []))
        (may (pay cost .once)
          (some (returnToHand this_ yourGraveyard))
          (some (exile this_)))]
}
