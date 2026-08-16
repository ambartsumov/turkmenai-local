// Load the real brand fonts so the video matches the product exactly. On Kaggle
// (Internet on) these fetch from Google Fonts at render time; the family names
// then resolve for the "Manrope" / "Space Mono" CSS declarations in theme.ts.
import { loadFont as loadManrope } from "@remotion/google-fonts/Manrope";
import { loadFont as loadSpaceMono } from "@remotion/google-fonts/SpaceMono";

loadManrope();
loadSpaceMono();
