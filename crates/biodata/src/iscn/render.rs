//! Canonical rendering from typed values.
use super::{
    Alternative, Band, Breakpoint, Chromosome, Clone, CloneBase, Count, Derivative, Event, Item,
    Karyotype, Mixture, Neo, Part, Rearrangement, RearrangementKind, SexChromosome, Site, SiteKind,
};
use std::fmt::{self, Display};

impl Display for Karyotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mixture {
            Some(Mixture::Mosaic) => f.write_str("mos ")?,
            Some(Mixture::Chimera) => f.write_str("chi ")?,
            None => {}
        }
        if self.donor_start == Some(0) {
            f.write_str("//")?;
        }
        for (index, clone) in self.clones.iter().enumerate() {
            if index > 0 {
                f.write_str(
                    if matches!(self.mixture, Some(Mixture::Chimera))
                        || self.donor_start.map(usize::from) == Some(index)
                    {
                        "//"
                    } else {
                        "/"
                    },
                )?;
            }
            write!(f, "{clone}")?;
        }
        if self.donor_start.map(usize::from) == Some(self.clones.len()) {
            f.write_str("//")?;
        }
        Ok(())
    }
}

impl Display for Clone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)?;
        if let Some(ploidy) = self.ploidy {
            write!(f, "<{ploidy}n>")?;
        }
        f.write_str(",")?;
        for chromosome in &self.sex {
            f.write_str(match chromosome {
                SexChromosome::X => "X",
                SexChromosome::Y => "Y",
            })?;
        }
        if self.sex_constitutional {
            f.write_str("c")?;
        }
        match self.base {
            Some(CloneBase::Idem) => f.write_str("idem")?,
            Some(CloneBase::Sl(number)) => {
                f.write_str("sl")?;
                if let Some(number) = number {
                    write!(f, "{number}")?;
                }
            }
            Some(CloneBase::Sdl(number)) => {
                f.write_str("sdl")?;
                if let Some(number) = number {
                    write!(f, "{number}")?;
                }
            }
            None => {}
        }
        if let Some(times) = self.base_times {
            write!(f, "x{times}")?;
        }
        for event in &self.events {
            write!(f, ",{event}")?;
        }
        if let Some(cells) = &self.cells {
            let prefix = if cells.composite { "cp" } else { "" };
            write!(f, "[{prefix}{}]", cells.count)?;
        }
        Ok(())
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.uncertain {
            f.write_str("?")?;
        }
        if let Some(sign) = self.sign {
            write!(f, "{sign}")?;
        }
        if let Some(count) = &self.prefix_count {
            write!(f, "{count}")?;
        }
        match &self.item {
            Item::Whole(chromosome) => write!(f, "{chromosome}")?,
            Item::Rearrangement(value) => write!(f, "{value}")?,
            Item::Derivative(value) => write!(f, "{value}")?,
            Item::Marker => f.write_str("mar")?,
            Item::Dmin => f.write_str("dmin")?,
            Item::Neo(value) => write!(f, "{value}")?,
        }
        if self.constitutional {
            f.write_str("c")?;
        }
        if let Some(copies) = &self.copies {
            let separator = if matches!(&self.item, Item::Rearrangement(value) if value.kind == RearrangementKind::Ring && value.parts.is_empty())
            {
                '×'
            } else {
                'x'
            };
            write!(f, "{separator}{copies}")?;
        }
        Ok(())
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Autosome(n) => write!(f, "{n}"),
            Self::X => f.write_str("X"),
            Self::Y => f.write_str("Y"),
            Self::Unknown => f.write_str("?"),
        }
    }
}

impl Display for Rearrangement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.uncertain {
            f.write_str("?")?;
        }
        if self.pseudo {
            f.write_str("psu ")?;
        }
        write!(f, "{}", self.kind)?;
        if self.ring {
            f.write_str(" r")?;
        }
        parts(f, &self.parts)
    }
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.site)?;
        if let Some(alternative) = &self.alternative {
            let (separator, site) = match alternative {
                Alternative::Or(site) => (" or ", site),
                Alternative::Range(site) => ("~", site),
            };
            f.write_str(separator)?;
            write!(f, "{site}")?;
        }
        Ok(())
    }
}

impl Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            SiteKind::Band(arm, band) => write!(f, "{arm}{band}"),
            SiteKind::Arm(arm) => write!(f, "{arm}?"),
            SiteKind::Terminal(arm) => write!(f, "{arm}ter"),
            SiteKind::Centromere => f.write_str("cen"),
            SiteKind::Unknown => f.write_str("?"),
        }?;
        if self.uncertain {
            f.write_str("?")?;
        }
        Ok(())
    }
}

impl Display for Band {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.region())?;
        if let Some(band) = self.band() {
            write!(f, "{band}")?;
        } else {
            f.write_str("?")?;
        }
        if let Some(sub) = self.sub() {
            write!(f, ".{sub}")?;
        }
        Ok(())
    }
}

impl Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(number) => write!(f, "{number}"),
            Self::Range(low, high) => write!(f, "{low}~{high}"),
            Self::About(number) => write!(f, "~{number}"),
        }
    }
}

impl Display for Derivative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.isoderivative { "ider" } else { "der" })?;
        parts(f, &self.parts)?;
        for change in &self.changes {
            write!(f, "{change}")?;
        }
        Ok(())
    }
}

impl Display for Neo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "neo({})({}->{}:)", self.chromosome, self.start, self.end)
    }
}

pub(crate) fn parts(f: &mut fmt::Formatter<'_>, parts: &[Part]) -> fmt::Result {
    if parts.is_empty() {
        return Ok(());
    }
    f.write_str("(")?;
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            f.write_str(";")?;
        }
        write!(f, "{}", part.chromosome)?;
    }
    f.write_str(")")?;
    if parts.iter().any(|part| !part.breakpoints.is_empty()) {
        f.write_str("(")?;
        for (index, part) in parts.iter().enumerate() {
            if index > 0 {
                f.write_str(";")?;
            }
            for breakpoint in &part.breakpoints {
                write!(f, "{breakpoint}")?;
            }
        }
        f.write_str(")")?;
    }
    Ok(())
}
