# binary-dev-generator-plane

`binary-dev-generator-plane` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies generator behavior through deny and allow fixtures, with explainable decision traces and local-only command execution.

## Why This Exists

This is intentionally local and self-contained so it can be inspected without credentials, services, or seeded history.

## Binary Dev Generator Plane Review Notes

`recovery` and `stale` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Capabilities

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/binary-dev-generator-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `safe rewrite` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Implementation Shape

The implementation keeps the scoring rule plain: reward signal and confidence, preserve slack, penalize drag, then classify the result into a review lane.

The Rust code keeps the review rule close to the tests.

## Local Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Verification

That command is also the regression path. It verifies the domain cases and catches mismatches between the CSV, metadata, and code.

## Roadmap

No external service is required. A deeper version would add more negative cases and a clearer boundary around invalid input.
