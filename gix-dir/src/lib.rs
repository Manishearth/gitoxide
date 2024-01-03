//! A crate for handling a git-style directory walk.
#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

/// A directory entry, typically obtained using [`walk()`].
pub struct Entry<'a> {
    /// The path at which the file or directory could be found.
    pub path: &'a Path,
    /// The kind of entry.
    pub kind: entry::Kind,
    pub mode: entry::Mode,
}

///
pub mod entry {

    /// The git-style filesystem mode.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub enum Mode {
        /// The entry is a blob, executable or not.
        Blob,
        /// The entry is a symlink.
        Symlink,
        /// The entry is an ordinary directory, which is either untracked or ignored along with all its contents.
        Directory,
        /// The entry is a directory which contains a `.git` folder.
        Submodule,
    }

    /// The kind of entry as obtained from a directory.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub enum Kind {
        /// The entry is not tracked by git yet, it was not found in the [index](gix_index::State).
        Untracked,
    }
}

///
pub mod walk {
    /// Options for use in [`walk()`](function::walk()) function.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub struct Options {
        /// If true, the filesystem will store paths as decomposed unicode, i.e. `ä` becomes `"a\u{308}"`, which means that
        /// we have to turn these forms back from decomposed to precomposed unicode before storing it in the index or generally
        /// using it. This also applies to input received from the command-line, so callers may have to be aware of this and
        /// perform conversions accordingly.
        /// If false, no conversions will be performed.
        pub precompose_unicode: bool,
        /// If true, the filesystem ignores the case of input, which makes `A` the same file as `a`.
        /// This is also called case-folding.
        pub ignore_case: bool,
    }

    /// Additional information collected as outcome of [`walk()`](function::walk()).
    #[derive(Debug, Clone, Copy)]
    pub struct Outcome {
        /// The amount of calls to read the directory contents.
        pub read_dir_calls: usize,
        /// The amount of returned entries.
        pub returned_entries: usize,
        /// The amount of entries, prior to pathspecs filtering them out or otherwise excluding them.
        pub seen_entries: usize,
    }

    /// The error returned by [`walk()`](function::walk()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    #[error("TBD")]
    pub enum Error {}

    pub(crate) mod function {
        use crate::walk::{Error, Options, Outcome};
        use std::path::Path;

        /// A function to perform a git-style directory walk.
        ///
        /// * `root` - the starting point of the walk and a readable directory.
        /// * `worktree_root` - the top-most root of the worktree, which must be a prefix to `root`.
        /// * `index` - access to see which files or directories are tracked.
        pub fn walk(
            root: &Path,
            worktree_root: &Path,
            _index: &gix_index::State,
            _options: Options,
        ) -> Result<Outcome, Error> {
            debug_assert!(
                root.strip_prefix(worktree_root).is_ok(),
                "{root:?} must be contained in {worktree_root:?}"
            );
            todo!()
        }
    }
}

use std::path::Path;
pub use walk::function::walk;
