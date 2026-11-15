### Fixed

- **ui:** The chat log no longer pulls a reader back to the bottom while a reply streams after they scroll up ("I can't scroll up while it's typing"), no longer undoes "Jump to latest" when an older follow finishes late or a scroll event arrives late, shows "New messages below" for content that arrived while it let go, and no longer stops short of the newest message when an older follow lands after a newer one. Found by a TLA+ model of the follow logic (`proofs/tla/ChatFollow.tla`), which CI now checks over every interleaving.
