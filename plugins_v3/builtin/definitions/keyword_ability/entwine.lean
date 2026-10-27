keyword entwine (cost : Cost) : Ability := {
  lexical := { lemma := "entwine", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): entwine [mana cost] / entwine—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.modeChoice (Semantics.Templates.ModeChange.allInstead (Option.some cost) Option.none))]
}
