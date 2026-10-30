/-- "Ward [cost]" means "Whenever this permanent becomes the target of a spell or ability an
opponent controls, counter that spell or ability unless that player pays [cost]." [CR#702.21a] -/
keyword ward (cost : Cost) : Ability := {
  lexical := { lemma := "ward", parameters := [[Cost]] },
  spelling := "ward <cost>",
  body :=
    [whenever
        (does
          (by_
            (a
              (and
                [in_ stack, opponentsControl])))
          (target thisPermanent))
        (instruct (controllerOf (that stack))
          (mayOrElse (pay cost once)
            (instruct you
              (counter (that stack)))))]
}
