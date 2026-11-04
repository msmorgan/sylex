# Tickets

## Folder layout

A ticket's status is the folder it lives in:

| Folder | Meaning |
|---|---|
| `critical/` | Engine happy path: the seams most abilities need |
| `planned/` | Committed work: Oracle-text coverage, keyword authoring, game-wide systems |
| `maybe/` | Speculative, or gated on a design discussion with the owner |
| `wip/` | Claimed and in progress |
| `done/` | Integrated into the default line |

Tickets move `critical/planned/maybe → wip → done`. The jj-kata folder-Kanban driver (configured
in `kata.toml`) makes the moves; never move a ticket by hand.

## Writing a ticket

Front matter carries `needs: [...]`: the slugs of tickets that must reach `done/` first. Run
`kata kanban check` after editing it. The body has up to four parts:

- **Why**: the problem, in a few sentences.
- **Done when**: checkable acceptance criteria, including which tests or gates must pass.
- **Not in scope**: only what a reader would likely confuse for part of the ticket.
- **Pointers**: the files or documents to read.

Never put in a ticket: "Standard constraints apply" or any restatement of `CLAUDE.md`; design
prose (a design decision lives in the ADR, the code or the declaration it shapes); provenance or
the history of the ticket or its predecessors; "(owner, date)" labels. Do not model a new ticket
on an existing one.
