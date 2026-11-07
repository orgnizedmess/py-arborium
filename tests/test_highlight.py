#!/usr/bin/env python3
import pytest
from arborium import Highlighter, HtmlFormat

SOURCE = """<html lang="en">
<head>
<style>
body { margin: 0 }
</style>
<body>
<p>Test</p>
</body>
</html>
"""

def test_default_highlighter():
    hl = Highlighter()
    source = """<html lang="en">
<head>
<style>
body { margin: 0 }
</style>
<body>
<p>Test</p>
</body>
</html>
"""
    html = hl.highlight("html", SOURCE)
    print(html)
    # HTML tags are rendered correctly
    assert "<a-p>&lt;</a-p><a-tg>html</a-tg> <a-at>lang</a-at>=&quot;<a-s>en</a-s>&quot;<a-p>&gt;</a-p>" in html
    # Inline CSS is also highlighted correctly
    assert "<a-pr>margin</a-pr>" in html

def test_unsupported_language():
    hl = Highlighter()
    with pytest.raises(ValueError) as e:
        result = hl.highlight("test", SOURCE)
    assert "unsupported language" in str(e.value)

def test_injection_depth():
    hl = Highlighter(max_injection_depth=0)
    html = hl.highlight("html", SOURCE)
    # Inline CSS is not highlighted
    assert "margin: 0" in html

def test_custom_prefix():
    hl = Highlighter(html_format=HtmlFormat.CustomElements(prefix="h"))
    html = hl.highlight("html", SOURCE)
    assert "<h-p>&lt;</h-p><h-tg>html</h-tg> <h-at>lang</h-at>=&quot;<h-s>en</h-s>&quot;<h-p>&gt;</h-p>" in html

def test_class_names_format():
    hl = Highlighter(html_format=HtmlFormat.ClassNames(prefix="hl"))
    html = hl.highlight("html", SOURCE)
    print(html)
    assert "<span class=\"hl-punctuation\">&lt;</span><span class=\"hl-tag\">html</span> <span class=\"hl-attribute\">lang</span>=&quot;<span class=\"hl-string\">en</span>&quot;<span class=\"hl-punctuation\">&gt;</span>" in html

def test_wrong_html_format():
    with pytest.raises(TypeError):
        Highlighter(html_format="class_names")
