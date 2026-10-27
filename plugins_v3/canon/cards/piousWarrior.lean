card piousWarrior : Spelled :=
  { name := "Pious Warrior", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [human, rebel, warrior],
    text :=
      [ whenever (does any (dealDamage (to := thisCreature) (kind := some .combat)))
          (gainLife thatMuch) ],
    power := stat 2, toughness := stat 3 }
