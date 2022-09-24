// #![extern crate sophia];
// extern crate sophia;

use std::fmt;
use std::error::Error;

use sophia::graph::{inmem::FastGraph, *};
// use sophia::dataset::{inmem::FastDataset, *};
use sophia::ns::Namespace;
use sophia::parser::turtle;
use sophia::serializer::nt::NtSerializer;
// use sophia::serializer::nq::QuadSerializer;
use sophia::serializer::*;
use sophia::triple::stream::TripleSource;

/// A nanopublication object
#[derive(Debug, Default)]
pub struct Nanopub {
    pub rdf: String,
    // graph
    // pubkey
    // privkey
    // orcid
    // server_url
    // publish: bool, // false
}
// https://docs.rs/sophia/0.5.3/sophia/dataset/inmem/index.html

impl Nanopub {
    /// Creates a new nanopub
    ///
    /// # Arguments
    ///
    /// * `rdf` - A string slice that holds the RDF of the nanopub
    ///
    /// # Examples
    ///
    /// ```
    /// use nanopub_rs::nanopub::Nanopub;
    /// let np = Nanopub::new("<http://s> <http://p> <http://o> .");
    /// ```
    pub fn new(rdf: &str) -> Result<Self, Box<dyn Error>> {
        // Self::default()

        let mut graph: FastGraph = turtle::parse_str(rdf).collect_triples()?;

        let ex = Namespace::new("http://example.org/")?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
        graph.insert(&ex.get("bob")?, &foaf.get("knows")?, &ex.get("alice")?)?;

        let mut nt_stringifier = NtSerializer::new_stringifier();
        // let mut nt_stringifier = QuadSerializer::new_stringifier();

        // let example2 = nt_stringifier.serialize_graph(&mut graph)?.as_str();
        // println!("The resulting graph\n{}", example2);

        Ok( Self {
            rdf: nt_stringifier.serialize_graph(&mut graph)?.to_string(),
            // graph: &mut graph,
        })

        // let np = match nt_stringifier.serialize_graph(&mut graph)?.to_string() {
        //     Ok(np) => np,
        //     Err(error) => panic!("Problem opening the file: {:?}", error),
        // };

        // Self {
        //     rdf: if let Some(rdf) = rdf {
        //         rdf.to_string()
        //     } else {
        //         "Default toast".to_string()
        //     }
        // }
    }

    // - preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear;
    // this includes the signature part, except the triple that is stating the actual signature
    // - preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
    // - Signature is calculated on this normalized representation
    // - Signature triple is added
    // - Trusty URI code is calculated on normalized representation that includes signature
    // - Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub


    /// Returns the RDF of the nanopub
    pub fn get_rdf(&self) -> String {
        self.rdf.clone()
    }

    // /// Returns all the quads contained by the nanopub.
    // pub fn iter(&self) -> Iter<'_> {
    //     let iter = self.spog.iter();
    //     Iter {
    //         dataset: self,
    //         inner: iter,
    //     }
    // }
}

impl fmt::Display for Nanopub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for t in self {
        //     writeln!(f, "{}", t)?;
        // }
        writeln!(f, "RDF: {}", self.rdf)?;
        Ok(())
    }
}
