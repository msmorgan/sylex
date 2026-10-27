/-- Vraska's Scorn -/
card vraskasScorn : Spelled :=
  { name := "Vraska's Scorn", cost := [generic 2, pip .black, pip .black], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ instruct (target isOpponent) (loseLife 4),
            may
              (sequentially
                [ searchLibraryOrGraveyard (a (named "Vraska, Scheming Gorgon")),
                  reveal it, putInto it hand ]),
            doIf (did you (search yourLibrary) thisWay) shuffle ]) ] }
