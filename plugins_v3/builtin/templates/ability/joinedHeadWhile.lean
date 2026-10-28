/-- A joined "whenever <event> while <concurrent>" header. -/
template joinedHeadWhile (event : GameEvent) (while_ : Timing) : JoinedHeader := {
  spelling := "whenever <event> <while_>",
  body :=
    { event := event, alternatives := [], while_ := some while_, timing := none }
}
