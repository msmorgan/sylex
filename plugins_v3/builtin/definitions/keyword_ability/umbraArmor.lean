keyword umbraArmor : Ability := {
  lexical := { lemma := "umbra armor" },
  spelling := "umbra armor",
  body :=
    [ static
        (replace (would any (destroy (enchanted .permanent)))
          (instead (sequentially [clearDamage (enchanted .permanent), destroy thisAura]))) ]
}
