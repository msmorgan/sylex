/-- Timely Reinforcements -/
card timelyReinforcements : Spelled :=
  { name := "Timely Reinforcements", cost := [generic 2, pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ doIf (compareAmt (lifeTotalOf you) .less (lifeTotalOf anOpponent))
              (gainLife 6),
            doIf (compareAmt (countOf (and [isCreature, youControl])) .less
                    (countOf (and [isCreature, anOpponentControls])))
              (create 3 (creatureToken 1 1 [.white] [soldier])) ]) ] }
