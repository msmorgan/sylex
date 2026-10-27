keyword spree : Ability := {
  lexical := { lemma := "spree" },
  spelling := "spree",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.modeChoice (Semantics.Templates.ModeChange.setQuantity (Semantics.Templates.Quantity.orMore 1)))]
}
