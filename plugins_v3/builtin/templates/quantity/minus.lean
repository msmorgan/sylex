template minus (left right : Amount) : Amount := {
  spelling := "<left> minus <right>",
  body := arith .minus left right
}
