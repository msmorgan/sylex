card harshSustenance : Spelled :=
  { name := "Harsh Sustenance", cost := [generic 1, pip .white, pip .black], types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage (letter .x) (target isAnyTarget)),
            gainLife (letter .x),
            define .x (countOf (and [isCreature, youControl])) ]) ] }
