### Translations that appear on the homepage

## index.html

index-title = The {halo-2} Proving System

what-is-halo-2 = What is {halo-2}?
what-is-halo-2-answer =
    {halo-2} is a proving system that combines the {$urlHalo}Halo recursion
    technique{$urlEnd} with an arithmetization based on {$urlPLONK}PLONK{$urlEnd},
    and a {$urlPCS}polynomial commitment scheme{$urlEnd} based around the Inner
    Product Argument. The protocol is described {$urlProtocol}here{$urlEnd}.
what-is-halo-2-impl = It is implemented across three Rust crates:

crate-desc-halo2_proofs =
    The core {halo-2} implementation. Provides the various traits and structs
    for writing circuits, and the APIs for creating and verifying proofs.
crate-desc-halo2_gadgets =
    A collection of reusable gadgets for building circuits, and implementations
    of chips to power them.
crate-desc-halo2 =
    Provides the APIs for building recursive circuits and proofs.
coming-soon = Coming soon!

circuit-description =
    {halo-2} circuits are two-dimensional: they use a grid of "cells" identified
    by columns and rows, into which values are assigned. Constraints on those
    cells are grouped into "gates", which apply to every row simultaneously, and
    can refer to cells at relative rows. To enable both low-level relative cell
    references in gates, and high-level layout optimisations, circuit developers
    can define "regions" in which assigned cells will preserve their relative
    offsets.

column-types = Column Types
column-type-instance =
    Instance columns contain per-proof public values, that the prover gives to
    the verifier.
column-type-advice =
    Advice columns are where the prover assigns private (witness) values, that
    the verifier learns zero knowledge about.
column-type-fixed =
    Fixed columns contain constants used by every proof that are baked into the
    circuit.
column-type-selector =
    Selector columns are special cases of fixed columns that can be used to
    selectively enable gates.

column-types-legend =
    In the example circuit layout pictured, the columns are indicated with
    different backgrounds. The instance column in white; advice columns in red;
    fixed columns in light blue; and selector columns in dark blue. Regions are
    shown in light green, and assigned cells in dark green or black.
