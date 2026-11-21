//! `scarlet-part`: the upload unit of Scarlet. One implementation of the
//! sharing groups, the part format, the wire batch and the decoder, used by the
//! desktop companion and by the server. See docs/DESIGN.md.

pub mod groups;
pub mod payload;
pub mod pseudo;
pub mod tables;
