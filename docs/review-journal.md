# Review Journal

The repository goal stays the same: build a Rust toolkit that studies generator behavior through deny and allow fixtures, with explainable decision traces and local-only command execution. This note explains the added review angle.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 182, lane `ship`
- `stress`: `diagnostic quality`, score 157, lane `ship`
- `edge`: `review cost`, score 204, lane `ship`
- `recovery`: `safe rewrite`, score 264, lane `ship`
- `stale`: `change width`, score 149, lane `ship`

## Note

This file is intentionally plain so the fixture remains the source of truth.
