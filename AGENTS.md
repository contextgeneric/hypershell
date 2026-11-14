# AGENTS.md: Hypershell

Follow these rules for every change to this repository. Read [README.md](README.md) for what the
project is and how to build it, and the knowledge base's
[sibling-projects.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/sibling-projects.md)
for the related repositories.

## Universal rules

These rules hold in every CGP repository. The
[knowledge base's AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/AGENTS.md)
holds their canonical wording: read it first, and change a rule there before updating this block:

- Learn CGP and its tools from the `/cgp` skill and the knowledge base; never restate them in this
  file.
- Prefer the local sibling checkout when reading another project.
- Write a cross-project link as a GitHub URL on `main`.
- Name official release versions only, never a pre-release.
- Commit only when the user explicitly asks, on the current branch.
- Verify a claim against the source before writing it.
- Keep every view of the same truth in sync in the same change.
- Ask the user when a decision is ambiguous.
- Apply point-first writing to internal prose.

## Task guides

The knowledge base documents this project and the website explains it, so a change to what either
describes starts from the document that holds it. Read it before the task:

- **Changing code the knowledge base describes:** the project's section,
  [projects/hypershell/](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/projects/hypershell/README.md),
  and the section's [writing guide](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/projects/writing-guide.md).
- **Changing code a website page explains:** the record of the site's section on it,
  [website/projects/hypershell.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/website/projects/hypershell.md).

## Checks before committing

Every commit passes the checks CI runs:

- Run the build and test commands in [README.md](README.md#building) before committing;
  [`.github/workflows/tests.yml`](.github/workflows/tests.yml) runs the same commands.
