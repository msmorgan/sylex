/-- Conqueror's Pledge -/
card conquerorsPledge : Spelled :=
  { name := "Conqueror's Pledge", cost := [generic 2, pip .white, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ kicker [generic 6],
        spell (replace
          (create 6 (creatureToken 1 1 [.white] [kor, soldier]))
          (doIf (keywordCostPaid kicker this_)
            (instead (create 12 asThose [])))) ] }
