# Specs

A spec is an accepted design that several tickets build and that is not built yet: too local for
a design doc ([`../design/README.md`](../design/README.md)) and with no code yet to carry it in
doc comments. A design only one ticket builds goes in that ticket's Design section instead. Each
ticket that builds a spec names it in its `specs:` front matter
([`../tickets/README.md`](../tickets/README.md)).

A spec marked `## Status: DRAFT` has not been built or checked against the code: it gives
direction, and what building it finds outranks its text.

A spec lives only until its design is in the code. The landing that builds it moves what stays
true into doc comments, tests or a design doc, and deletes the spec file.
