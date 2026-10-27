/-- Ballot Broker -/
card ballotBroker : Spelled :=
  { name := "Ballot Broker", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [human, advisor],
    text := [static (can you (vote (times := additional (exactly 1))))],
    power := stat 2, toughness := stat 3 }
