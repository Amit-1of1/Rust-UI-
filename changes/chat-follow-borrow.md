### Fixed

- **ui (experimental `chat`):** `ChatLog` no longer panics with "already borrowed" when two follow-to-bottom passes overlap (seen in the Android webview, where scroll queries are slow): the last offset is copied out before awaiting the next one. (#68)
