card chainReaction : Spelled :=
  { name := "Chain Reaction", cost := [generic 2, pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage (letter .x) (each isCreature)),
            define .x (countOf isCreature) ]) ] }
