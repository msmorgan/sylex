/-- "Shroud": "This permanent or player can't be the target of spells or abilities"
[CR#702.18a], written over its holder (`this_`). -/
keyword shroud : Ability := {
  lexical := { lemma := "shroud" },
  spelling := "shroud",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any (Semantics.Templates.Action.target Semantics.Templates.NounPhrase.this_))]
}
