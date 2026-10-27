keyword bestow (cost : Cost) : Ability := {
  lexical := { lemma := "bestow", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): bestow [mana cost] / bestow—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static (altCost this_ (some cost))]
}
