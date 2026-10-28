keyword ravenous : Ability := {
  lexical := { lemma := "ravenous" },
  spelling := "ravenous",
  body :=
    [static
        (entersWithCounters thisPermanent
          (letter Semantics.Letter.x) p1p1Counter),
      when
        (does (actor thisPermanent) enter)
        (draw 1) [] [] none none none
        (some
          (compareAmt (letter Semantics.Letter.x) Semantics.Comparator.atLeast 5))]
}
