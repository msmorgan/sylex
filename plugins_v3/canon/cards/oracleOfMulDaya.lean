card oracleOfMulDaya : Spelled :=
  { name := "Oracle of Mul Daya", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [elf, shaman],
    text :=
      [ static
          (can you (play (counted (additional (exactly 1)) isLand))
             (timing := onEachOfYourTurns)),
        static (visibility .reveal you topOfLibrary),
        static (can you (play (allOf isLand) (from_ := onTop))) ],
    power := stat 2, toughness := stat 2 }
