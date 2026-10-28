/-- A State-Based Action Conferral: if the condition holds, the game performs the Action
[CR#704.1,704.3]. References to this Object denote the holder of the Registry Definition. -/
template stateBased (when : Condition) (then_ : Action) : Conferral := {
  body :=
    .stateBased when then_
}
