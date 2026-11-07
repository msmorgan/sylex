# Specs

A spec is a design that has been accepted but is not built yet: too local for an ADR
([`../decisions/README.md`](../decisions/README.md)) and with no code yet to carry it in doc
comments. Tickets that build it name it in their `specs:` front matter
([`../tickets/README.md`](../tickets/README.md)).

A spec lives only until its design is in the code. The landing that builds it moves what stays
true into doc comments, tests or an ADR, and deletes the spec file.
