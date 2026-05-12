# Quickstart: Ordered Artifact Filenames

## Verify artifact ordering

1. Initialize a test repository with `canon init`.
2. Author a requirements brief under `canon-input/requirements.md`.
3. Run `canon run --mode requirements --risk bounded-impact --zone yellow --owner product-lead --input canon-input/requirements.md`.
4. List the artifacts: `ls .canon/artifacts/<RUN_ID>/requirements/`.
5. Verify every file starts with a two-digit prefix and the order matches the intended reading order.

## Verify publish preserves ordering

1. Complete and approve the run from the quickstart above.
2. Run `canon publish <RUN_ID>`.
3. List the published directory and verify prefixed filenames are preserved.

## Verify manifest references

1. After a completed architecture run, read `.canon/artifacts/<RUN_ID>/architecture/view-manifest.json`.
2. Verify all artifact paths use the `NN-` prefix.
