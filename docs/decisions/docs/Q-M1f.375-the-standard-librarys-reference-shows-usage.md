# Q-M1f.375 — The standard library's reference shows usage apart from how a module works, and every rule drawn

The reference of the standard library is a folder, `docs/ein/user/04-reference/04-stdlib/`, a page a family of
modules, and each family is written in two parts that are kept apart.

- **Using it** comes first: what a program imports and lists, every relation, macro and entry name it writes, and
  what each produces, the facts derived, the answer's shape, what is flagged. Each is shown by a small complete
  program with what `ein` prints for it, the failures among them.
- **How it works** comes after, under its own heading, or on the page that follows where the rules are many: the
  module's rules by name and how its modules stack. **A rule that is listed is shown**: as Ein, the rule as its
  module writes it, and as the drawing `ein render rule` writes for it. A rule whose drawing is too large to read,
  a closing rule that names every relation and rule of its module, is shown as Ein alone.

Every module of the library has one page that says how to use it, and every rule of every module is shown once.

**Why.** A reader who writes a program wants to know what to type and what comes back, and reads the rules only
when an answer surprises. Mixed, each gets in the other's way. A rule described in a table's cell is a
paraphrase, which drifts from the rule and cannot be checked against it; the rule's own text and the engine's
drawing of it are the rule, and a drawing is how a pattern over a graph is read.

**Held by.** The pages of the folder, whose index, [`README.md`](../../ein/user/04-reference/04-stdlib/README.md),
lists each module with its two pages. The text and the drawing of a rule on a page are generated from the module's
file and the engine, and are not edited by hand; the drawing's source is a file of the folder's `diagrams/`,
named for the page, the module and the rule, and the page shows the picture drawn from it
([Q-M1f.368](Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md)).
