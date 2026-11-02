//! Preparsed bundles: every `.syl` file under one plugin directory, read and
//! parsed once.
//!
//! A [`Bundle`] is what later layers (`semantics`) consume instead of reading
//! disk themselves: each file's [`SourceFile`] keeps its path, its text and its
//! syntax tree behind an [`Arc`], so the layers share one parse. The bundle
//! maps paths to files in a deterministic order and resolves a [`FileId`]
//! (and so any [`Span`](crate::Span)) back to its file for `path:line:column`
//! diagnostics.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::Error;
use crate::parse::parse;
use crate::span::FileId;
use crate::span::Span;
use crate::tree::File;

/// One parsed `.syl` file.
#[derive(Debug)]
pub struct SourceFile {
    pub id: FileId,
    /// The path the file was read from: the bundle's root joined with the
    /// file's path inside it.
    pub path: PathBuf,
    pub source: String,
    pub ast: File,
}

impl SourceFile {
    /// `path:line:column` of the start of `span`, which must lie in this file.
    #[must_use]
    pub fn location(&self, span: Span) -> String {
        format!(
            "{}:{}",
            self.path.display(),
            span.start_line_col(&self.source)
        )
    }
}

/// The `.syl` files of one plugin directory, such as `plugins_v3/builtin`.
#[derive(Debug)]
pub struct Bundle {
    root: PathBuf,
    /// Keyed by the path relative to `root`.
    files: BTreeMap<PathBuf, Arc<SourceFile>>,
    /// `files`' values, indexed by `FileId` minus `first`.
    by_id: Vec<Arc<SourceFile>>,
    first: FileId,
}

/// Why a bundle could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("reading {}: {error}", path.display())]
    Io {
        path: PathBuf,
        error: std::io::Error,
    },
    /// Every file that failed to parse, each error rendered as
    /// `path:line:column: message`.
    #[error("{}", rendered.join("\n"))]
    Parse {
        errors: Vec<(PathBuf, Error)>,
        rendered: Vec<String>,
    },
}

impl Bundle {
    /// Reads and parses every `.syl` file under `root`, recursively. Files are
    /// numbered in path order starting at `first`, so bundles loaded with
    /// disjoint ranges ([`Bundle::next_file_id`]) never share a `FileId`.
    ///
    /// # Errors
    ///
    /// If a directory or file cannot be read, or any file fails to parse; a
    /// parse failure reports every file that failed.
    ///
    /// # Panics
    ///
    /// If the file ids would overflow `u32`.
    pub fn load(root: impl AsRef<Path>, first: FileId) -> Result<Arc<Bundle>, LoadError> {
        let root = root.as_ref();
        let mut paths = Vec::new();
        collect(root, &mut paths)?;
        let mut sources = Vec::with_capacity(paths.len());
        for path in paths {
            let source = std::fs::read_to_string(&path).map_err(|error| LoadError::Io {
                path: path.clone(),
                error,
            })?;
            let relative = path
                .strip_prefix(root)
                .expect("collected under root")
                .to_path_buf();
            sources.push((relative, source));
        }
        Bundle::from_sources(root, sources, first)
    }

    /// Parses in-memory sources as a bundle rooted at `root`; each path is
    /// relative to `root`. Nothing is read from disk.
    ///
    /// # Errors
    ///
    /// If any source fails to parse.
    ///
    /// # Panics
    ///
    /// If the file ids would overflow `u32`.
    pub fn from_sources(
        root: impl AsRef<Path>,
        sources: impl IntoIterator<Item = (PathBuf, String)>,
        first: FileId,
    ) -> Result<Arc<Bundle>, LoadError> {
        let root = root.as_ref().to_path_buf();
        let sorted: BTreeMap<PathBuf, String> = sources.into_iter().collect();
        let mut files = BTreeMap::new();
        let mut by_id = Vec::with_capacity(sorted.len());
        let mut errors = Vec::new();
        let mut rendered = Vec::new();
        for (index, (relative, source)) in sorted.into_iter().enumerate() {
            let id = FileId(first.0 + u32::try_from(index).expect("fewer than 2^32 files"));
            let path = root.join(&relative);
            match parse(id, &source) {
                Ok(ast) => {
                    let file = Arc::new(SourceFile {
                        id,
                        path,
                        source,
                        ast,
                    });
                    by_id.push(Arc::clone(&file));
                    files.insert(relative, file);
                }
                Err(error) => {
                    rendered.push(error.render(&path.display().to_string(), &source));
                    errors.push((path, error));
                }
            }
        }
        if !errors.is_empty() {
            return Err(LoadError::Parse { errors, rendered });
        }
        Ok(Arc::new(Bundle {
            root,
            files,
            by_id,
            first,
        }))
    }

    /// The directory the bundle was loaded from.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The files, keyed by their path relative to [`Bundle::root`], in path
    /// order.
    #[must_use]
    pub fn files(&self) -> &BTreeMap<PathBuf, Arc<SourceFile>> {
        &self.files
    }

    /// The file a [`FileId`] names, if it belongs to this bundle.
    #[must_use]
    pub fn file(&self, id: FileId) -> Option<&Arc<SourceFile>> {
        let index = id.0.checked_sub(self.first.0)?;
        self.by_id.get(usize::try_from(index).ok()?)
    }

    /// The first [`FileId`] after this bundle's: where the next bundle's
    /// numbering may start.
    ///
    /// # Panics
    ///
    /// If the file ids would overflow `u32`.
    #[must_use]
    pub fn next_file_id(&self) -> FileId {
        FileId(self.first.0 + u32::try_from(self.by_id.len()).expect("fewer than 2^32 files"))
    }
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), LoadError> {
    let io = |error| LoadError::Io {
        path: dir.to_path_buf(),
        error,
    };
    for entry in std::fs::read_dir(dir).map_err(io)? {
        let path = entry.map_err(io)?.path();
        if path.is_dir() {
            collect(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "syl") {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_files_in_path_order_from_first() {
        let bundle = Bundle::from_sources(
            "root",
            [
                (PathBuf::from("b.syl"), "template b: Int { body = 1 }".to_string()),
                (PathBuf::from("a.syl"), "template a: Int { body = 1 }".to_string()),
            ],
            FileId(5),
        )
        .unwrap();
        let ids: Vec<_> = bundle.files().values().map(|f| f.id).collect();
        assert_eq!(ids, [FileId(5), FileId(6)]);
        assert_eq!(bundle.file(FileId(6)).unwrap().path, Path::new("root/b.syl"));
        assert!(bundle.file(FileId(4)).is_none());
        assert!(bundle.file(FileId(7)).is_none());
        assert_eq!(bundle.next_file_id(), FileId(7));
    }

    #[test]
    fn reports_every_parse_failure_with_its_location() {
        let error = Bundle::from_sources(
            "root",
            [
                (PathBuf::from("a.syl"), "template a: Int { body = $ }".to_string()),
                (PathBuf::from("b.syl"), "template b: Int {\n  body = -}".to_string()),
            ],
            FileId(0),
        )
        .unwrap_err();
        let LoadError::Parse { rendered, .. } = error else {
            panic!("expected a parse error");
        };
        assert_eq!(rendered.len(), 2);
        assert!(rendered[0].starts_with("root/a.syl:1:"), "{}", rendered[0]);
        assert!(rendered[1].starts_with("root/b.syl:2:"), "{}", rendered[1]);
    }

    #[test]
    fn loads_the_builtin_plugins() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/builtin");
        let bundle = Bundle::load(&root, FileId(0)).unwrap();
        assert!(bundle.files().len() > 1000);
        let first = bundle.files().values().next().unwrap();
        assert_eq!(first.id, FileId(0));
        assert!(Arc::ptr_eq(bundle.file(first.id).unwrap(), first));
    }
}
