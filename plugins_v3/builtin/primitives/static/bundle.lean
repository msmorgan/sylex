/-- The characteristics a quality change adds or sets; "It's still a <retained>" keeps every
prior type [CR#205.1b]. -/
-- [CR#205.1b]
primitive bundle (characteristics : CharacteristicBundle) (retained : Option CardType := none) :
    QualityPayload := {
  body :=
    Semantics.Primitives.QualityPayload.bundle characteristics retained
}
