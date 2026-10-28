/-- Sagas use lore counters to track progress [CR#714.3]. Chapter abilities trigger as the count
crosses their threshold [CR#714.2b]; the final chapter number also governs the Saga's sacrifice
[CR#714.4]. -/
counter loreCounter := {
  lexical := { lemma := "lore counter" },
  spelling := "lore counter",
  holder := .object,
  confers := []
}
