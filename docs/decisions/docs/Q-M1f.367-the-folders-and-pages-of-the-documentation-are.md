# Q-M1f.367 — The folders and pages of the documentation are numbered in reading order

Every folder and page under the four parts of `docs/ein/`, which are `user/`, `reasoning/`, `ein.rs/` and
`overview/`, is named `NN-kebab-name`: `NN` is its place in its folder's reading order, from `01`, with no gap and
no number twice, and the name after it is lower case, its words joined by `-`. A folder's `README.md` keeps its
name and lists the folder's entries in that order. A folder of a page's assets, `diagrams/`, keeps its name, and
the four parts keep theirs. A page that is added takes the number of its place, and the pages after it move up,
every reference to them with them.

**Why.** A listing of a folder then reads in the order its pages are meant to be read, in a file browser and in a
terminal as on the rendered index, and a reader who lands on one page knows what comes before it and what after
without opening the index. A gap would say a page is missing and a number twice that two pages are alternatives,
so neither is written.

**Held by.** The tree, from `docs/ein/user/01-install.md` to `docs/ein/ein.rs/07-reserved-engine-strings.md`; each
folder's `README.md`, which lists its entries in their numbered order; and
[`docs/ein/README.md`](../../ein/README.md), which names the four parts.
