# py-arborium

Python bindings for [arborium]: a syntax highlighter powered by tree-sitter, grammars included.

[arborium]: https://arborium.bearcove.eu

## Building from source

Requirements: Rust 1.90 or newer, Python 3.10 or newer, and [uv](https://docs.astral.sh/uv/).

```bash
git clone https://github.com/orgnizedmess/py-arborium
cd py-arborium
uv sync
uv run pytest tests
```

The first build compiles every bundled grammar, so it takes a while and uses a lot of memory. Later builds are much faster.

To build a wheel for use in another project:

```bash
uvx maturin build --release
# In the other project directory
uv add <py-arborium-path>/target/wheels/<wheel-file>
```

## Usage

```python
from arborium import Highlighter

hl = Highlighter()
hl.highlight("rust", "fn main() { println!(\"{}\", \"Hello World\")}")
hl.highlight("python", "print('hello')")
# <a-f>print</a-f>(<a-s>&#39;hello&#39;</a-s>)
```

Render `span` tags with class names instead of custom HTML elements:

```python
hl = Highlighter(html_format=HtmlFormat.ClassNames())
print(hl.highlight("python", "print('hello')"))
# <span class="function">print</span>(<span class="string">&#39;hello&#39;</span>)
```

Provide a custom prefix for custom elements or class names:

```python
hl = Highlighter(html_format=HtmlFormat.ClassNames(prefix="hl"))
hl.highlight("python", "print('hello')")
# <span class="hl-function">print</span>(<span class="hl-string">&#39;hello&#39;</span>)
```
