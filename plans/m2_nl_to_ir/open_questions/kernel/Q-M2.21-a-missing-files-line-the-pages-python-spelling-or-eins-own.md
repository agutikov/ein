# Q-M2.21 — A missing file's line: the page's Python spelling, or Ein's own words?

**Status:** open

**Raised 2026-11-25 by S2.1.4,
its design point 2**, which carries E-74
and the missing file's half of [F23](../../../followups/f23_diagnostics_in_eins_words.md)'s D-1. S2.1.4 ran with the
default (its record, `df75643c`): one line, the page's, from the library
and every command; this is what the default leaves for the user.

**The question.** A missing file is spelled three ways today: every command but one prints the page's line through
`ein_ir::read`, `FileNotFoundError: [Errno 2] No such file or directory: 'nope.ein'`; the library's
`ein_ir::load_file` prints `nope.ein: No such file or directory (os error 2)`; and `ein test nope.ein` prints
`error: nope.ein: No such file or directory (os error 2)`. S2.1.4 makes it one line, made in one place. Which line:
the one [`07-diagnostics.md` § 2](../../../../docs/ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes)
states today, in Python's words, or a line in Ein's own?

**The default taken**, S2.1.4's design point 2: *"**The line is the page's**,
`FileNotFoundError: [Errno 2] No such file or directory: '<path>'`, so no `.expected` fixture moves; replacing it
with a line in Ein's own words is D-1's other half, and shown to the user as the alternative (it moves the page's
row and every fixture and golden that holds the line)."* Its risks say the same: *"Making it one line is the bug;
making it Ein's own words is a design question of F23's, shown, not taken."*

**What the default costs.** Ein's reference and every surface keep a Python exception's name for a refusal of Ein's,
and F23's D-1 stays open for it, beside the page's two other Python spellings (`IsADirectoryError`,
`PermissionError`, § 2.18), which S2.1.4 does not take.

**The alternatives.**

1. **The page's line, everywhere** (the default): the bug is fixed, the words stay; no fixture moves.
2. **A line in Ein's own words, everywhere**, in the same stage: `07-diagnostics.md` § 2's first row, and every
   `.expected` fixture and golden that holds the line, move with it; F23's D-1 closes for the missing file.
3. **The same, with § 2.18's two other spellings**: D-1 closed whole, the three read-path refusals in one voice.

**Where the answer lands**: it is about Ein, so a decision under `docs/decisions/`, written with S2.1.4's change.
The kind does not move either way: every one of these is `unreadable`.
