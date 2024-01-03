use gix_testtools::scripted_fixture_read_only;

mod baseline {
    use std::path::Path;

    /// Parse multiple walks out of a single `fixture`.
    pub fn extract_walks(_fixture: &Path) -> crate::Result {
        Ok(())
    }
}

#[test]
fn baseline() -> crate::Result {
    baseline::extract_walks(&scripted_fixture_read_only("walk_baseline.sh")?)?;
    Ok(())
}

#[test]
#[ignore = "to be implemented"]
fn precompose_unicode() {}

#[test]
#[ignore = "need case-insensitive testing as well"]
fn case_insensitive_usecases() {}

#[test]
#[ignore = "what about partial checkouts?"]
fn partial_checkouts() {}

#[test]
#[ignore = "what about submodules - are they treated specifically?"]
fn submodules() {}
