use pyo3::prelude::*;

/// Python bindings for `arborium`.
///
/// Arborium provides batteries-included syntax highlighting powered by
/// tree-sitter - https://arborium.bearcove.eu
#[pymodule(name = "arborium")]
mod py_arborium {
    use arborium::{Error, HtmlFormat};
    use pyo3::exceptions::{PyIOError, PyRuntimeError, PyValueError};
    use pyo3::prelude::*;

    /// HTML output format for a `Highlighter`.
    ///
    /// Formats: `CustomElements` and `ClassNames`, each with an optional
    /// prefix.
    #[derive(Clone)]
    #[pyclass(name = "HtmlFormat", frozen, from_py_object)]
    enum PyHtmlFormat {
        #[pyo3(constructor = (prefix: "str | None"=None))]
        CustomElements { prefix: Option<String> },
        #[pyo3(constructor = (prefix: "str | None"=None))]
        ClassNames { prefix: Option<String> },
    }

    impl From<PyHtmlFormat> for HtmlFormat {
        fn from(arg: PyHtmlFormat) -> Self {
            match arg {
                PyHtmlFormat::CustomElements { prefix } => match prefix {
                    Some(p) => HtmlFormat::CustomElementsWithPrefix(p),
                    None => HtmlFormat::CustomElements,
                },
                PyHtmlFormat::ClassNames { prefix } => match prefix {
                    Some(p) => HtmlFormat::ClassNamesWithPrefix(p),
                    None => HtmlFormat::ClassNames,
                },
            }
        }
    }

    /// High-level syntax highlighter for HTML output.
    ///
    /// Args:
    ///     max_injection_depth: Maximum depth for processing language
    ///         injections.
    ///
    ///         - 0: No injections (just primary language)
    ///         - 3: Default, handles most cases (HTML with CSS/JS, Markdown with code blocks)
    ///         - Higher: For deeply nested content
    ///
    ///     html_format: HTML output format.
    #[pyclass]
    struct Highlighter {
        inner: arborium::Highlighter,
    }

    #[pymethods]
    impl Highlighter {
        #[new]
        #[pyo3(signature = (max_injection_depth: "str | None"=None, html_format: "HtmlFormat | None"=None))]
        pub fn new(max_injection_depth: Option<u32>, html_format: Option<PyHtmlFormat>) -> Self {
            let mut config = arborium::Config::default();

            if let Some(depth) = max_injection_depth {
                config.max_injection_depth = depth;
            }

            if let Some(format) = html_format {
                config.html_format = format.into();
            }

            Self {
                inner: arborium::Highlighter::with_config(config),
            }
        }

        /// Highlight source code and return HTML string.
        ///
        /// Args:
        ///    lang: Name of the language to highlight. View the Rust
        ///        documentation for the list of supported languages.
        ///    source: The source code to highlight.
        ///
        /// Returns:
        ///      str: The highlighted HTML.
        ///
        ///  Raises:
        ///      ValueError: If the requested language is unsupported.
        ///      OSError: If an I/O error occurs while highlighting.
        ///      RuntimeError: If the error occured while compiling the
        ///          tree-sitter query or while parsing the source code.
        pub fn highlight(&mut self, lang: &str, source: &str) -> PyResult<String> {
            let result = self.inner.highlight(lang, source);
            match result {
                Ok(code) => Ok(code),
                Err(e) => match e {
                    Error::Io(_) => Err(PyIOError::new_err(e.to_string())),
                    Error::UnsupportedLanguage { .. } => Err(PyValueError::new_err(e.to_string())),
                    _ => Err(PyRuntimeError::new_err(e.to_string())),
                },
            }
        }
    }
}
