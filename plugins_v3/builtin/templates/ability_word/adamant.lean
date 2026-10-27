/- ## Ability words

An ability word has no rules meaning [CR#207.2c]: each is a Template, the italic word over the
ability it heads, and no Definition. -/

ability_word adamant (ability : Ability) : Ability := {
  lexical := { lemma := "adamant", listed := Rules },
  label := "adamant",
  spelling := "adamant — <ability>",
  body :=
    ability
}
