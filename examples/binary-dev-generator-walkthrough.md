# Binary Dev Generator Plane Walkthrough

This note is the quickest way to read the extra review model in `binary-dev-generator-plane`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 182 | ship |
| stress | diagnostic quality | 157 | ship |
| edge | review cost | 204 | ship |
| recovery | safe rewrite | 264 | ship |
| stale | change width | 149 | ship |

Start with `recovery` and `stale`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The useful comparison is `safe rewrite` against `change width`, not the raw score alone.
