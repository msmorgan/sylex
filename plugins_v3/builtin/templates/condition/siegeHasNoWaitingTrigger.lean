/-- "It isn't the source of an ability that has triggered but not yet left the stack", over the
row's own subject [CR#704.5v,310.7,113.7]. -/
template siegeHasNoWaitingTrigger : Condition := {
  spelling := "it isn't the source of an ability that has triggered but not yet left the stack",
  body :=
    .not (exists_ (.and [.abilityOf .this, triggeredNotLeftStack]))
}
