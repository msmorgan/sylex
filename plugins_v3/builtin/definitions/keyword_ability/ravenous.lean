keyword ravenous : Ability := {
  lexical := { lemma := "ravenous" },
  spelling := "ravenous",
  body :=
    [ static (entersWithCounters thisPermanent (.letter .x) p1p1Counter),
      when (does thisPermanent enter)
        (if_ := some (.compareAmt (.letter .x) .atLeast 5))
        (draw 1) ]
}
