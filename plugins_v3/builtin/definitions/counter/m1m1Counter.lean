/-- [CR#122.1a]: the minus counter's twin of `p1p1CounterDefinition`, subtracting 1 from each
stat. -/
counter m1m1Counter := {
  lexical := { lemma := "-1/-1 counter" },
  spelling := "-1/-1 counter",
  holder := .object,
  confers := [boost (.down 1) (.down 1)]
}
