# Make every plain public record that derives Deserialize refuse a positional JSON array, through one shared declaration, so each record's reader accepts exactly the object its schema states

Status: closed.
Promoted: 0019

Kind: idea
When: After the BioData 1.0.0 tag

## Problem

Make every plain public record that derives Deserialize refuse a positional JSON array, through one shared declaration, so each record's reader accepts exactly the object its schema states. Every BioData reader then matches its object schema, as the validated values already do from 1.0.
