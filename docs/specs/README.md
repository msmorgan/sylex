# Specs

A spec is an accepted design that several tickets build and that is not built yet: too local for
an ADR ([`../decisions/README.md`](../decisions/README.md)) and with no code yet to carry it in
doc comments. A design only one ticket builds goes in that ticket's Design section instead. Each
ticket that builds a spec names it in its `specs:` front matter
([`../tickets/README.md`](../tickets/README.md)).

A spec lives only until its design is in the code. The landing that builds it moves what stays
true into doc comments, tests or an ADR, and deletes the spec file.
