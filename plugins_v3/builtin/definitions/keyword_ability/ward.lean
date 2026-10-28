keyword ward (cost : Cost) : Ability := {
  lexical := { lemma := "ward", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): ward [mana cost] / ward—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [whenever
        (does
          (by_
            (a
              (and
                [in_ stack, opponentsControl])))
          (target thisPermanent))
        (instruct (controllerOf (that Semantics.NounWord.stack))
          (mayOrElse (pay cost Semantics.PayTimes.once)
            (instruct you
              (counter (that Semantics.NounWord.stack)))))]
}
