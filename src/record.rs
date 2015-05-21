impl RecordType {
    pub fn parse_record_type(record_string: &str) -> Option<RecordType> {
        match record_string {
            "CRYST1" => Some(RecordType::CRYST1),
            "END"    => Some(RecordType::END),
            "HEADER" => Some(RecordType::HEADER),
            "NUMMDL" => Some(RecordType::NUMMDL),
            "MASTER" => Some(RecordType::MASTER),
            "ORIGXn" => Some(RecordType::ORIGXn),
            "SCALEn" => Some(RecordType::SCALEn),
            "AUTHOR" => Some(RecordType::AUTHOR),
            "CAVEAT" => Some(RecordType::CAVEAT),
            "COMPND" => Some(RecordType::COMPND),
            "EXPDTA" => Some(RecordType::EXPDTA),
            "MDLTYP" => Some(RecordType::MDLTYP),
            "KEYWDS" => Some(RecordType::KEYWDS),
            "OBSLTE" => Some(RecordType::OBSLTE),
            "SOURCE" => Some(RecordType::SOURCE),
            "SPLIT"  => Some(RecordType::SPLIT),
            "SPRSDE" => Some(RecordType::SPRSDE),
            "TITLE"  => Some(RecordType::TITLE),
            "ANISOU" => Some(RecordType::ANISOU),
            "ATOM"   => Some(RecordType::ATOM),
            "CISPEP" => Some(RecordType::CISPEP),
            "CONECT" => Some(RecordType::CONECT),
            "DBREF"  => Some(RecordType::DBREF),
            "HELIX"  => Some(RecordType::HELIX),
            "HET"    => Some(RecordType::HET),
            "HETATM" => Some(RecordType::HETATM),
            "LINK"   => Some(RecordType::LINK),
            "MODRES" => Some(RecordType::MODRES),
            "MTRIXn" => Some(RecordType::MTRIXn),
            "REVDAT" => Some(RecordType::REVDAT),
            "SEQADV" => Some(RecordType::SEQADV),
            "SHEET"  => Some(RecordType::SHEET),
            "SSBOND" => Some(RecordType::SSBOND),
            "FORMUL" => Some(RecordType::FORMUL),
            "HETNAM" => Some(RecordType::HETNAM),
            "HETSYN" => Some(RecordType::HETSYN),
            "SEQRES" => Some(RecordType::SEQRES),
            "SITE"   => Some(RecordType::SITE),
            "ENDMDL" => Some(RecordType::ENDMDL),
            "MODEL"  => Some(RecordType::MODEL),
            "TER"    => Some(RecordType::TER),
            "JRNL"   => Some(RecordType::JRNL),
            "REMARK" => Some(RecordType::REMARK),
            _ => None,
        }
    }
}

pub enum RecordType {
    /// Unit cell parameters, space group, and Z.
    CRYST1, 
    /// Last record in the file.
    END,    
    /// First line of the entry, contains PDB ID code, classification, and date of deposition.
    HEADER,
    /// Number of models.
    NUMMDL,
    /// Control record for bookkeeping.
    MASTER,
    /// Transformation from orthogonal  coordinates to the submitted coordinates (n = 1, 2, or 3).
    ORIGXn,
    /// Transformation from orthogonal coordinates to fractional crystallographic coordinates 
    /// (n = 1, 2, or 3).
    SCALEn,
    /// List of contributors.
    AUTHOR,
    /// Severe error indicator.
    CAVEAT,
    /// Description of macromolecular contents of the entry.
    COMPND,
    /// Experimental technique used for the structure determination.
    EXPDTA,
    /// Contains additional annotatio  pertinent to the coordinates presented in the entry.
    MDLTYP,
    /// List of keywords describing the macromolecule.
    KEYWDS,
    /// Statement that the entry has been removed from distribution and list of the ID code(s)
    /// which replaced it.
    OBSLTE,
    /// Biological source of macromolecules in the entry.
    SOURCE,
    /// List of PDB entries that compose a larger macromolecular complexes.
    SPLIT,
    /// List of entries obsoleted from public release and replaced by current entry.
    SPRSDE,
    /// Description of the experiment represented in the entry.
    TITLE,
    /// Anisotropic temperature factors.
    ANISOU,
    /// Atomic coordinate records for standard groups.
    ATOM,
    /// Identification of peptide residues in cis conformation.
    CISPEP,
    /// Connectivity records.
    CONECT,
    /// Reference to the entry in the sequence database(s).
    DBREF,
    /// Identification of helical substructures.
    HELIX,
    /// Identification of non-standard groups heterogens).
    HET,
    /// Atomic coordinate records for heterogens.
    HETATM,
    /// Identification of inter-residue bonds.
    LINK,
    /// Identification of modifications to standard residues.
    MODRES,
    /// Transformations expressing non-crystallographic symmetry (n = 1, 2, or 3). There may be
    /// multiple sets of these records.
    MTRIXn,
    /// Revision date and related information.
    REVDAT,
    /// Identification of conflicts between PDB and the named sequence database.
    SEQADV,
    /// Identification of sheet substructures.
    SHEET,
    /// Identification of disulfide bonds.
    SSBOND,
    /// Chemical formula of non-standard groups.
    FORMUL,
    /// Compound name of the heterogens.
    HETNAM,
    /// Synonymous compound names for heterogens.
    HETSYN,
    /// Primary sequence of backbone residues.
    SEQRES,
    /// Identification of groups comprising important entity sites.
    SITE,
    /// End-of-model record for multiple structures in a single coordinate entry.
    ENDMDL,
    /// Specification of model number for multiple structures in a single coordinate entry.
    MODEL,
    /// Chain terminator.
    TER,
    /// Literature citation that defines the coordinate set.
    JRNL,
    /// General remarks; they can be structured or free form.
    REMARK,
}
