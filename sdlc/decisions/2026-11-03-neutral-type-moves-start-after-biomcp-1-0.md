# Neutral type moves start after BioMCP 1.0

Date: 2026-11-03. Decided by: BioData PM, accepted by BioMCP's PM.

The moves the [neutral types list](../designs/2026-11-01-neutral-types-left-in-biomcp.md) names start after BioMCP 1.0 is tagged, so BioMCP's release candidate and its schema baseline see no change. Drug regulatory values move first, and no module is held back.

Each move is one BioData ticket per module that ships in a BioData minor release. One BioMCP ticket then repins, swaps its imports, and shows an empty reply diff and an unchanged schema comparison. BioMCP's PM accepts each BioData ticket before it is built.
