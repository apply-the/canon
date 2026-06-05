# Quickstart: Policy Shaping Mode

## Creating a Policy Change
1. Create a branch and initialize the policy-shaping mode.
2. Draft your rule in `draft-policy.md`.
3. Run the CLI assessment:
   ```bash
   canon policy-shaping draft-policy.md
   ```
4. Review the generated `conformance-impact-report.md`. If the blast radius is too large, it will be paginated, and you will need explicit broad-impact approval.
5. Review the generated `04-migration.md`. Adjust the debt scheduling and waivers as needed.
6. Provide Systemic Impact sign-off using `--approve`:
   ```bash
   canon policy-shaping draft-policy.md --approve
   ```
7. Review the generated `policy-diff.md` and submit your Pull Request.
