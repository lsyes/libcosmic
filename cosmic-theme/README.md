# cosmic-theme

Theme library and default palettes for the COSMIC desktop.

## Default palette

The built-in light and dark palettes (`src/model/light.ron`, `src/model/dark.ron`)
follow the [Material Design 3](https://m3.material.io/) tonal color system
(`material_color_utilities` HCT tonal-40/80 roles), so the derived themes match
the common "Material 3 baseline" look:

| Palette field        | Light                    | Dark                     | MD3 role               |
| -------------------- | ------------------------ | ------------------------ | ---------------------- |
| `gray_1` (background)| `#FEF7FF`                | `#141218`                | surface               |
| `gray_2`             | `#F3EDF7`                | `#0F0D13`                | surfaceContainer(-Lowest) |
| `neutral_0..10`      | white → black            | black → white            | neutral tonal ramp     |
| `bright_red`         | `#B3261E`                | `#F2B8B5`                | error                  |
| `bright_green`       | `#2E7D32`                | `#81C995`                | success (green tone)   |
| `bright_orange`      | `#B25000`                | `#FFB77C`                | warning (amber tone)   |
| `accent_blue`        | `#6750A4`                | `#D0BCFF`                | primary (default accent) |
| `accent_indigo`      | `#625B71`                | `#CCC2DC`                | secondary              |
| `accent_purple`      | `#7D5260`                | `#EFB8C8`                | tertiary               |
| `accent_pink`        | `#BA1A5F`                | `#FFB1C8`                | pink tone              |
| `accent_red`         | `#B3261E`                | `#F2B8B5`                | error                  |
| `accent_orange`      | `#B25000`                | `#FFB77C`                | amber tone             |
| `accent_yellow`      | `#8F6F00`                | `#E3CA3E`                | yellow tone            |
| `accent_green`       | `#2E7D32`                | `#81C995`                | green tone             |
| `accent_warm_grey`   | `#7A757E`                | `#948F99`                | neutral-variant tone   |
| `ext_*`              | vibrant brand colors     | vibrant brand colors     | extended palette (icons/branding) |

The neutral ramps keep the palette convention `neutral_0` is the extreme tone of
the theme's own mode (darkest in dark, lightest in light) and `neutral_10` is
the opposite extreme. All other theme colors (surfaces, containers, components,
text, dividers) are derived from this palette in `ThemeBuilder::build`.

## Testing snapshots

The qt5ct/qt6ct QPalette output snapshots are regenerated with:

```sh
INSTA_UPDATE=always cargo test -p cosmic-theme
```