/-- Burn at the Stake -/
card burnAtTheStake : Spelled :=
  { name := "Burn at the Stake", cost := [generic 2, pip .red, pip .red, pip .red],
    types := [.sorcery],
    text :=
      [ static (addedCost
          (tap (counted anyNumber (and [isCreature, youControl, isUntapped])))
          false),
        spell (instruct this_ (dealDamage (times 3 groupSize) (target isAnyTarget))) ] }
