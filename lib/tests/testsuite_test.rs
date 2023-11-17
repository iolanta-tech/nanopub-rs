use nanopub::{Nanopub, NpProfile};
use std::{error::Error, fs, path::Path};

const ORCID: &str = "http://orcid.org/0000-0000-0000-0000";

// #[test]
// fn publish_testsuite_valid_plain() -> Result<(), Box<dyn Error>> {
//     let private_key = fs::read_to_string("./tests/resources/id_rsa").unwrap();
//     let path = Path::new("tests/testsuite/valid/plain");
//     // Iterate over files
//     for entry in fs::read_dir(path)? {
//         let file = entry?;
//         let filename = format!("{:?}", file.file_name());
//         if filename.ends_with("trig\"") {
//             println!("\n☑️  Testing file: {}", filename);
//             let profile = NpProfile::new(ORCID, "", &private_key, None)?;
//             let np_rdf = fs::read_to_string(file.path())?;
//             let np = Nanopub::publish(&np_rdf, &profile, None)?;
//             // assert!(np.published);
//         }
//     }
//     Ok(())
// }

#[test]
fn check_valid_signed() {
    let np_rdf =
        fs::read_to_string("./tests/testsuite/valid/signed/Darwin-Core-schema-resource.trig")
            .unwrap();
    Nanopub::check(&np_rdf).expect("Failed check");
    // let np_rdf = fs::read_to_string("./tests/testsuite/valid/signed/python-step-1.trig").unwrap();
    // Nanopub::check(&np_rdf).expect("Failed check");
}
// TODO: check tests/testsuite/valid/signed
// check tests/testsuite/valid/trusty

// def test_testsuite_valid_signed():
//     test_files = Path("./tests/testsuite/valid/signed").rglob('*')

//     for test_file in test_files:
//         test_file = Path("tests/testsuite/valid/signed/simple1-rsa.trig")
//         print(f'☑️ Testing valid plain nanopub: {test_file}')
//         # if "/signed." in str(test_file):
//         #     continue

//         np = Nanopub(
//             conf=testsuite_conf,
//             rdf=test_file
//         )
//         np.sign()
//         print(np)
//         assert np.is_valid
//         assert np.metadata.trusty is not None
//         assert np.metadata.signature is not None
//         assert java_wrap.check_trusty_with_signature(np)
//         # TODO: we should be able to validate this signature?
//         # assert np.has_valid_signature
//         break

// def test_testsuite_valid_trusty():
//     test_files = Path("./tests/testsuite/valid/trusty").rglob('*')

//     for test_file in test_files:
//         print(f'☑️ Testing valid trusty nanopub: {test_file}')
//         np = Nanopub(
//             conf=testsuite_conf,
//             rdf=test_file
//         )
//         assert np.is_valid
//         assert np.metadata.trusty is not None

// def test_testsuite_sign_valid():
//     """Test to sign various files from transform and valid folder.
//     Compare the generated trusty URI to the one generated by nanopub-java"""
//     test_files = [
//         "./tests/testsuite/transform/signed/rsa-key1/simple1.in.trig",
//         "./tests/testsuite/transform/trusty/aida1.in.trig",
//         "./tests/testsuite/transform/trusty/simple1.in.trig",
//         "./tests/testsuite/valid/plain/aida1.trig",
//         "./tests/testsuite/valid/plain/simple1.nq",
//         "./tests/testsuite/valid/plain/simple1.trig",
//         "./tests/testsuite/valid/plain/simple1.xml",
//     ]

//     for test_file in test_files:
//         print(f'✒️ Testing signing valid nanopub: {test_file}')
//         np_g = ConjunctiveGraph()
//         if test_file.endswith(".xml"):
//             np_g.parse(test_file, format="trix")
//         else:
//             np_g.parse(test_file)

//         np = Nanopub(
//             conf=testsuite_conf,
//             rdf=np_g
//         )
//         java_np = java_wrap.sign(np)
//         np.sign()
//         assert np.has_valid_signature
//         assert np.has_valid_trusty
//         assert np.is_valid
//         assert java_wrap.check_trusty_with_signature(np)
//         assert np.source_uri == java_np

// def test_testsuite_valid_signature():
//     test_files = [
//         "./tests/testsuite/valid/signed/simple1-signed-rsa.trig",
//         # "./tests/testsuite/valid/signed/simple1-signed-dsa.trig",
//     ]
//     # java -jar lib/nanopub-1.38-jar-with-dependencies.jar sign tests/testsuite/transform/signed/rsa-key1/simple1.in.trig

//     for test_file in test_files:
//         print(f'✅ Testing validating signed valid nanopub: {test_file}')
//         np = Nanopub(
//             conf=testsuite_conf,
//             rdf=Path(test_file)
//         )
//         assert np.is_valid
//         assert np.has_valid_signature
//         assert np.has_valid_trusty
//         assert java_wrap.check_trusty_with_signature(np)

// def test_testsuite_invalid_plain():
//     test_files = Path("./tests/testsuite/invalid/plain").rglob('*')

//     for test_file in test_files:
//         print(f'❎ Testing validating invalid nanopub: {test_file}')
//         try:
//             np = Nanopub(
//                 conf=testsuite_conf,
//                 rdf=Path(test_file)
//             )
//             np.is_valid
//             assert False
//         except MalformedNanopubError as e:
//             print(e)
//             assert True
