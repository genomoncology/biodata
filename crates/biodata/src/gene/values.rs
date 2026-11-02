use crate::canonical_enum;
canonical_enum! {
    /// HGNC approval status.
    GeneStatus, GeneStatusUnknown {
        /// An HGNC approved record.
        Approved => "approved",
        /// An HGNC withdrawn record.
        Withdrawn => "withdrawn",
    }
}
canonical_enum! {
    /// HGNC locus group, separate from a missing source value.
    LocusGroup, LocusGroupUnknown {
        /// Protein coding.
        ProteinCoding => "protein_coding",
        /// Non-coding RNA.
        NonCodingRna => "non_coding_rna",
        /// Pseudogene.
        Pseudogene => "pseudogene",
        /// Phenotype.
        Phenotype => "phenotype",
        /// HGNC's stated other group.
        Other => "other",
    }
}
canonical_enum! {
    /// How a source matched a gene input.
    MatchKind, MatchKindUnknown {
        /// Current symbol.
        Symbol => "symbol",
        /// Previous symbol.
        PreviousSymbol => "previous_symbol",
        /// HGNC alias.
        Alias => "alias",
        /// NCBI-only alias.
        NcbiAlias => "ncbi_alias",
        /// HGNC identifier.
        HgncId => "hgnc_id",
        /// NCBI Gene identifier.
        EntrezId => "entrez_id",
        /// Ensembl gene identifier.
        EnsemblId => "ensembl_id",
        /// Cytogenetic band.
        Band => "band",
    }
}
canonical_enum! {
    /// The attribution on a source-provided summary, independent of fetch providers.
    SummarySource, SummarySourceUnknown {
        /// RefSeq attribution.
        RefSeq => "refseq",
    }
}
canonical_enum! {
    /// HGNC's stated withdrawn-file status.
    HgncStatus, HgncStatusUnknown {
        /// Entry withdrawn.
        EntryWithdrawn => "entry_withdrawn",
        /// Entry merged or split.
        MergedSplit => "merged_split",
    }
}

canonical_enum! {
    /// HGNC locus type; Unknown is a source-stated value, distinct from absence.
    LocusType, LocusTypeUnknown {
        /// HGNC gene with protein product.
        GeneWithProteinProduct => "gene_with_protein_product",
        /// HGNC rna cluster.
        RnaCluster => "rna_cluster",
        /// HGNC long non coding rna.
        LongNonCodingRna => "long_non_coding_rna",
        /// HGNC micro rna.
        MicroRna => "micro_rna",
        /// HGNC misc rna.
        MiscRna => "misc_rna",
        /// HGNC ribosomal rna.
        RibosomalRna => "ribosomal_rna",
        /// HGNC small nuclear rna.
        SmallNuclearRna => "small_nuclear_rna",
        /// HGNC small nucleolar rna.
        SmallNucleolarRna => "small_nucleolar_rna",
        /// HGNC transfer rna.
        TransferRna => "transfer_rna",
        /// HGNC vault rna.
        VaultRna => "vault_rna",
        /// HGNC y rna.
        YRna => "y_rna",
        /// HGNC phenotype only.
        PhenotypeOnly => "phenotype_only",
        /// HGNC pseudogene.
        Pseudogene => "pseudogene",
        /// HGNC complex locus constituent.
        ComplexLocusConstituent => "complex_locus_constituent",
        /// HGNC endogenous retrovirus.
        EndogenousRetrovirus => "endogenous_retrovirus",
        /// HGNC fragile site.
        FragileSite => "fragile_site",
        /// HGNC immunoglobulin gene.
        ImmunoglobulinGene => "immunoglobulin_gene",
        /// HGNC immunoglobulin pseudogene.
        ImmunoglobulinPseudogene => "immunoglobulin_pseudogene",
        /// HGNC readthrough.
        Readthrough => "readthrough",
        /// HGNC region.
        Region => "region",
        /// HGNC t cell receptor gene.
        TCellReceptorGene => "t_cell_receptor_gene",
        /// HGNC t cell receptor pseudogene.
        TCellReceptorPseudogene => "t_cell_receptor_pseudogene",
        /// HGNC transposable element.
        TransposableElement => "transposable_element",
        /// HGNC unknown.
        Unknown => "unknown",
        /// HGNC virus integration site.
        VirusIntegrationSite => "virus_integration_site",
    }
}
