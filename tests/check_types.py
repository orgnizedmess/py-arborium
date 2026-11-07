from arborium import Highlighter, HtmlFormat

Highlighter(html_format="oops")   # type: ignore[arg-type]
Highlighter(max_injection_depth="3") # type: ignore[arg-type]
Highlighter().highlight("rust") # type: ignore[call-arg]
Highlighter(html_format=HtmlFormat.ClassNames(prefix="hl-"))
