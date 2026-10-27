/-- Churning Eddy -/
card churningEddy : Spelled :=
  { name := "Churning Eddy", cost := [generic 3, pip .blue], types := [.sorcery],
    text := [ spell (returnToHand (both (target isCreature) (target isLand))) ] }
