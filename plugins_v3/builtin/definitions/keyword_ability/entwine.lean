keyword entwine (cost : Cost) : Ability := {
  lexical := { lemma := "entwine", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): entwine [mana cost] / entwine—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static (.modeChoice (.allInstead (some cost) none))]
}
