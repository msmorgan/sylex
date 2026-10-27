card punctureBlast : Spelled :=
  { name := "Puncture Blast", cost := [generic 2, pip .red], types := [.instant],
    text := [wither, spell (instruct this_ (dealDamage 3 (target isAnyTarget)))] }
