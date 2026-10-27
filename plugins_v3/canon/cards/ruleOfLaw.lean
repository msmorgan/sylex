/-- Rule of Law -/
card ruleOfLaw : Spelled :=
  { name := "Rule of Law", cost := [generic 2, pip .white], types := [.enchantment],
    text := [static (cant eachPlayer (cast (counted (moreThan 1) isSpell))
      (timing := eachTurn))] }
