# Canon Plugin Logo And Icon Correction

## Scope

Correct the vector symbol used by:

- `assistant/assets/canon-plugin-icon.svg`
- `assistant/assets/canon-plugin-logo.svg`

The change is limited to those two SVG assets.

## Design

Both assets use the same vector symbol: a thick outer hexagonal ring surrounding
an enlarged, right-facing `C`. The symbol uses purple gradients plus
semi-transparent polygon overlays to reproduce the faceted depth visible in the
approved reference image.

The SVGs remain native vectors. They do not embed the raster reference image.

## Asset-Specific Behavior

- `canon-plugin-icon.svg` contains the symbol on a transparent background.
- `canon-plugin-logo.svg` contains the same symbol and the existing white
  `canon` wordmark on a transparent background.

## Preserved Behavior

- Existing asset paths remain unchanged.
- Existing SVG view boxes remain unchanged.
- The logo wordmark typography, position, and color remain unchanged.
- The reference image background is intentionally excluded.

## Verification

- Render both SVGs and inspect them visually.
- Confirm that both SVGs remain valid XML.
- Run the repository-required formatting and Clippy checks because files in the
  repository changed.
