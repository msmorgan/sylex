keyword tiered : Ability := {
  lexical := { lemma := "tiered" },
  spelling := "tiered",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.modeChoice (Semantics.Templates.ModeChange.setQuantity (Semantics.Templates.Quantity.oneThrough 1)))]
}
