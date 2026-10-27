/-- "<subj> <action>s as though <condition>": an indicative statement that the act happens as
though the condition were true, for that event only [CR#609.4]; not a permission, which is
`can` [CR#101.2]. -/
-- [CR#609.4]
-- [CR#101.2]
primitive asThough (subj : Subject) (action : Action) (condition : AsThough) :
    StaticSpec := {
  -- TODO(spelling): <subj> <action>s as though <condition>; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    Semantics.Primitives.StaticSpec.asThough subj action condition
}
