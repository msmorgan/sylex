card callInAProfessional : Spelled :=
  { name := "Call In a Professional", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (cant (bare isPlayer) (gainLife anyAmount)) thisTurn,
            establishThroughout
              (cant any (preventDamage allDamage (damage := would any (dealDamage anyAmount))))
              thisTurn,
            instruct this_ (dealDamage 3 (target isAnyTarget)) ]) ] }
