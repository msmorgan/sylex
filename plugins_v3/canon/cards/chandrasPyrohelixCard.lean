card chandrasPyrohelixCard : Spelled :=
  { name := "Chandra's Pyrohelix", cost := [generic 1, pip .red], types := [.instant],
    text := [spell (dealDivided this_ 2 (targets (oneThrough 2) isAnyTarget))] }
