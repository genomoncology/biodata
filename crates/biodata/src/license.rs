crate::canonical_enum! {
    /// License reported by the bucket metadata record.
    License, LicenseUnknown {
        /// Creative Commons Attribution.
        CcBy => "cc_by",
        /// Creative Commons Attribution NonCommercial.
        CcByNc => "cc_by_nc",
        /// Creative Commons Attribution ShareAlike.
        CcBySa => "cc_by_sa",
        /// Creative Commons Attribution NonCommercial ShareAlike.
        CcByNcSa => "cc_by_nc_sa",
        /// Creative Commons Attribution NoDerivatives.
        CcByNd => "cc_by_nd",
        /// Creative Commons Attribution NonCommercial NoDerivatives.
        CcByNcNd => "cc_by_nc_nd",
        /// Creative Commons public-domain dedication.
        Cc0 => "cc0",
        /// Public domain.
        PublicDomain => "public_domain",
        /// Text and data mining manuscript terms.
        Tdm => "tdm",
        /// The bucket reports another license.
        Other => "other",
    }
}
