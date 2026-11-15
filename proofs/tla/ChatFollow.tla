---------------------------- MODULE ChatFollow ----------------------------
(***************************************************************************)
(* A model of `Follow` in crates/ui/src/components/chat/component.rs: the *)
(* chat log keeps the newest message in view while the reader is at the   *)
(* end, and lets go when they scroll up.                                  *)
(*                                                                         *)
(* `Follow::run` is spawned after every render, on every resize (each     *)
(* streamed token) and on mount, so several runs are often in flight at   *)
(* once. Dioxus runs tasks on one thread; code interleaves only at        *)
(* `.await`. A run is four stretches, each ending at an await:            *)
(*   size    `get_scroll_size()` returned: record the height; stop if not *)
(*           pinned                                                        *)
(*   offset  `get_scroll_offset()` returned: if the reader is above where *)
(*           the log last left them, let go; else ask to scroll down      *)
(*   scroll  `scroll()` returned: the view has moved                       *)
(*   after   `get_scroll_offset()` returned: remember where we left them  *)
(* Meanwhile messages arrive, the reader scrolls, and on the web scroll   *)
(* events arrive late; Blitz sends none (Events = FALSE).                  *)
(*                                                                         *)
(* Positions are whole steps, each bigger than the code's SLACK: `h` is   *)
(* the scroll height, `V` the viewport, `off` the scroll offset, and the  *)
(* end of the log is End(h) = h - V.                                       *)
(*                                                                         *)
(* The Old* constants put back five of the six bugs TLC found in the     *)
(* code; scripts/tla.sh requires each to be caught. ChatFollow.cfg checks *)
(* follow with 3 messages; ChatFollowJump.cfg adds "Jump to latest".     *)
(***************************************************************************)
EXTENDS Integers, FiniteSets

CONSTANTS
    MaxH,            \* how far the content can grow
    V,               \* the viewport's height
    MaxReader,       \* scrolls the reader may make (keeps the model finite)
    MaxJumps,        \* "Jump to latest" clicks
    Events,          \* TRUE on the web: scroll events arrive, late
    OldStaleTop,     \* old code: read `top` before awaiting the offset
    OldTrustAfter,   \* old code: record whatever offset follows the scroll
    OldScrollToSize, \* old code: scroll to the height read at the start
    OldNoEpoch,      \* old code: runs started before "Jump to latest" carry on after it
    OldKeepSeenTop   \* old code: "Jump to latest" keeps where scroll events last saw the reader

None == -1   \* Option::None
Runs == 1..(MaxH + MaxJumps + 1)
Pcs == {"idle", "size", "offset", "scroll", "after", "done"}
End(hh) == IF hh > V THEN hh - V ELSE 0
Min(a, b) == IF a < b THEN a ELSE b

VARIABLES
    h, off,                 \* the page: scroll height, scroll offset
    pinned, unseen,         \* signals
    height, top, seenTop,   \* signals: last height, where the log left the reader, last event
    run,                    \* each run task: where it is and its locals
    spawned,                \* runs started so far
    evq,                    \* a scroll event is waiting to be delivered (web)
    readerUp,               \* ghost: the reader scrolled up and hasn't come back to the end
    upTracked,              \* ghost: ... above where the log last left them (`top`)
    yanked,                 \* ghost: a run pulled down a reader it knew had scrolled up
    missed,                 \* ghost: content arrived that the reader hasn't seen
    upSincePin,             \* ghost: the reader scrolled up since the log last pinned
    reads, jumps            \* ghost: counters

vars == <<h, off, pinned, unseen, height, top, seenTop, run, spawned, evq,
          readerUp, upTracked, yanked, missed, upSincePin, reads, jumps>>

Opt(S) == S \cup {None}
Run == [pc : Pcs, size : 0..MaxH, grew : BOOLEAN, last : Opt(0..MaxH), sawUp : BOOLEAN,
        ep : 0..MaxJumps]
Fresh == [pc |-> "idle", size |-> 0, grew |-> FALSE, last |-> None, sawUp |-> FALSE, ep |-> 0]
\* A finished run keeps no locals, so runs that ended differently are one state (smaller model).
Done == [Fresh EXCEPT !.pc = "done"]

Init ==
    /\ h = 0 /\ off = 0
    /\ pinned = TRUE /\ unseen = FALSE
    /\ height = 0 /\ top = None /\ seenTop = None
    /\ run = [r \in Runs |-> IF r = 1 THEN [Fresh EXCEPT !.pc = "size"] ELSE Fresh] \* onmounted
    /\ spawned = 1
    /\ evq = FALSE
    /\ readerUp = FALSE /\ upTracked = FALSE /\ yanked = FALSE /\ missed = FALSE
    /\ upSincePin = FALSE
    /\ reads = 0 /\ jumps = 0

\* The view moved: on the web that queues a scroll event (they coalesce).
Moved(newOff) == evq' = (Events /\ (evq \/ newOff # off))

\* A view at the end shows everything.
Sees(newOff, newH) == missed' = (missed /\ newOff # End(newH))

----------------------------------------------------------------------------
(* The outside world *)

\* A message arrives or a streaming reply grows: the log re-renders and runs a follow.
Grow ==
    /\ h < MaxH /\ spawned < Cardinality(Runs)
    /\ h' = h + 1
    /\ run' = [run EXCEPT ![spawned + 1].pc = "size", ![spawned + 1].ep = jumps]
    /\ spawned' = spawned + 1
    /\ missed' = (missed \/ off # End(h + 1))
    /\ UNCHANGED <<off, pinned, unseen, height, top, seenTop, evq, readerUp, upTracked, upSincePin,
                   yanked, reads, jumps>>

ReaderUp ==
    /\ reads < MaxReader /\ off > 0
    /\ \E o \in 0..(off - 1) :
        /\ off' = o
        /\ Moved(o)
        /\ upTracked' = (upTracked \/ (top # None /\ o < top))
    /\ readerUp' = TRUE
    /\ reads' = reads + 1
    /\ UNCHANGED <<h, pinned, unseen, height, top, seenTop, run, spawned, yanked, missed, jumps>>
    /\ upSincePin' = TRUE

ReaderEnd ==
    /\ reads < MaxReader /\ off < End(h)
    /\ off' = End(h)
    /\ Moved(End(h))
    /\ readerUp' = FALSE /\ upTracked' = FALSE /\ missed' = FALSE
    /\ reads' = reads + 1
    /\ UNCHANGED <<h, pinned, unseen, height, top, seenTop, run, spawned, yanked, upSincePin, jumps>>

\* The `onscroll` handler (web only), reading the element when it runs.
Deliver ==
    /\ evq
    /\ evq' = FALSE
    /\ LET atEnd == off = End(h)
           before == IF seenTop # None THEN seenTop ELSE top
           movedUp == before # None /\ off < before
       IN /\ seenTop' = off
          /\ pinned' = IF atEnd /\ ~pinned THEN TRUE
                       ELSE IF ~atEnd /\ movedUp /\ pinned THEN FALSE
                       ELSE pinned
          /\ unseen' = IF atEnd THEN FALSE ELSE unseen
          /\ upSincePin' = (upSincePin /\ ~atEnd)
    /\ UNCHANGED <<h, off, height, top, run, spawned, readerUp, upTracked, yanked, missed,
                   reads, jumps>>

\* The "Jump to latest" button (`Follow::jump`).
Jump ==
    /\ ~pinned /\ jumps < MaxJumps /\ spawned < Cardinality(Runs)
    /\ pinned' = TRUE /\ unseen' = FALSE /\ top' = None
    /\ run' = [run EXCEPT ![spawned + 1].pc = "size", ![spawned + 1].ep = jumps + 1]
    /\ spawned' = spawned + 1
    /\ readerUp' = FALSE /\ upTracked' = FALSE
    /\ jumps' = jumps + 1
    \* A scroll event still queued from before the click must not count as the reader moving up
    \* (TLC found it unpinning right after "Jump to latest"), so forget where events last saw them.
    /\ seenTop' = IF OldKeepSeenTop THEN seenTop ELSE None
    /\ UNCHANGED <<h, off, height, evq, yanked, missed, reads>>
    /\ upSincePin' = FALSE

----------------------------------------------------------------------------
(* One run of `Follow::run`, a stretch at a time *)

Keep == UNCHANGED <<h, spawned, readerUp, upTracked, missed, upSincePin, reads, jumps, seenTop>>

StepSize(r) ==
    /\ run[r].pc = "size"
    /\ LET size == h
           grew == size > height
       IN /\ height' = size
          /\ IF ~pinned
                THEN /\ unseen' = (unseen \/ grew)
                     /\ run' = [run EXCEPT ![r] = Done]
                ELSE /\ unseen' = unseen
                     /\ run' = [run EXCEPT ![r] = [@ EXCEPT !.pc = "offset", !.size = size,
                                                           !.grew = grew,
                                                           !.last = IF OldStaleTop THEN top ELSE None]]
    /\ Keep /\ UNCHANGED <<off, pinned, top, evq, yanked>>

\* Old bug 1 (OldStaleTop): `top` was copied out before this await, so a run that started
\* before another run recorded its first position still held None, skipped the check, and
\* pulled down a reader who had scrolled up meanwhile.
StepOffset(r) ==
    /\ run[r].pc = "offset"
    /\ LET last == IF OldStaleTop THEN run[r].last ELSE top
       IN IF last # None /\ off < last
             THEN /\ pinned' = FALSE
                  /\ unseen' = (unseen \/ run[r].grew)
                  /\ run' = [run EXCEPT ![r] = Done]
             ELSE /\ run' = [run EXCEPT ![r].pc = "scroll", ![r].sawUp = upTracked]
                  /\ UNCHANGED <<pinned, unseen>>
    /\ Keep /\ UNCHANGED <<off, height, top, evq, yanked>>

\* Old bug 3 (OldScrollToSize): the scroll asked for the height read at the start, so a run
\* that read it before new content arrived could finish after a newer run and leave the log
\* above the end. The fix asks for far past the end, which every engine clamps to the end.
StepScroll(r) ==
    /\ run[r].pc = "scroll"
    /\ LET to == IF OldScrollToSize THEN Min(run[r].size, End(h)) ELSE End(h)
       IN /\ off' = to
          /\ Moved(to)
          /\ yanked' = (yanked \/ (upTracked /\ run[r].sawUp /\ to > off))
          /\ Sees(to, h)
          \* A scroll asked for before the reader moved still lands: a race no code can close
          \* (the check and the landing are a round trip apart). The reader is at the end again.
          /\ readerUp' = (readerUp /\ to # End(h))
          /\ upTracked' = (upTracked /\ to # End(h))
    /\ run' = [run EXCEPT ![r].pc = "after"]
    /\ UNCHANGED <<h, spawned, reads, jumps, seenTop, pinned, unseen, height, top, upSincePin>>

\* Old bug 2 (OldTrustAfter): whatever offset came back was recorded as where the log left
\* the reader. If they scrolled up between the scroll and this read, that was their own
\* position, and every later run compared against it, saw no movement and pulled them down.
\* The fix: the scroll landed at the end, which is at least End(size) and at least the last
\* position recorded (content only grows while pinned), so an offset above either is the
\* reader, and the log lets go. (Comparing with `top` too matters when this run is stale: a newer
\* run may already have recorded a lower bound higher than this run's End(size).)
Floor(r) == IF top # None /\ top > End(run[r].size) THEN top ELSE End(run[r].size)

StepAfter(r) ==
    /\ run[r].pc = "after"
    /\ run' = [run EXCEPT ![r] = Done]
    /\ IF ~OldTrustAfter /\ off < Floor(r)
          THEN /\ pinned' = FALSE /\ top' = top
               \* Content another run measured after this one's scroll is news to the reader.
               /\ unseen' = (unseen \/ height > run[r].size)
          ELSE /\ top' = off /\ pinned' = pinned /\ unseen' = unseen
    /\ Keep /\ UNCHANGED <<off, height, evq, yanked>>

\* The fifth bug TLC found: a run started before "Jump to latest" could finish after it and let
\* go again (its last step saw the reader's old position), undoing the click. The fix: `jump`
\* starts a new epoch, and a run from an older epoch stops at its next await.
Stale(r) ==
    /\ run[r].pc \in {"size", "offset", "after"}
    /\ run' = [run EXCEPT ![r] = Done]
    /\ UNCHANGED <<h, off, pinned, unseen, height, top, seenTop, spawned, evq, readerUp,
                   upTracked, yanked, missed, upSincePin, reads, jumps>>

\* A scroll already asked for lands whatever the epoch; the code's checks are what stop.
Step(r) ==
    \/ StepScroll(r)
    \/ IF OldNoEpoch \/ run[r].ep = jumps
          THEN StepSize(r) \/ StepOffset(r) \/ StepAfter(r)
          ELSE Stale(r)

Next ==
    \/ Grow \/ ReaderUp \/ ReaderEnd \/ Jump
    \/ (Events /\ Deliver)
    \/ \E r \in Runs : Step(r)

Spec == Init /\ [][Next]_vars /\ \A r \in Runs : WF_vars(Step(r)) /\ WF_vars(Events /\ Deliver)

----------------------------------------------------------------------------
(* What must hold *)

TypeOK ==
    /\ h \in 0..MaxH /\ off \in 0..End(h)
    /\ pinned \in BOOLEAN /\ unseen \in BOOLEAN
    /\ height \in 0..MaxH /\ top \in Opt(0..MaxH) /\ seenTop \in Opt(0..MaxH)
    /\ run \in [Runs -> Run]

\* Nothing is left to happen until the next message or scroll.
Quiet == (\A r \in Runs : run[r].pc \in {"idle", "done"}) /\ ~evq

\* A run that looks after the reader scrolled above where the log last left them never pulls
\* them back down. That is the promise in the code's comment, and all a log without scroll
\* events can know: before the first follow (after mounting or "Jump to latest") there is no
\* position to compare with, and a reader back exactly where the log left them looks the same
\* as one who never moved.
NoYank == ~yanked

\* At rest, a pinned log that the reader didn't leave shows the end.
Follows == Quiet /\ pinned /\ ~readerUp => off = End(h)

\* At rest, a reader away from the end who hasn't seen new content is told so.
TellsOfNew == Quiet /\ ~pinned /\ missed => unseen

\* The log only lets go because the reader scrolled up: a stale run never unpins it by itself
\* (which would show "Jump to latest" to a reader who never moved).
UnpinsOnlyForReader == ~pinned => upSincePin

\* Every run finishes.
RunsEnd == \A r \in Runs : (run[r].pc \notin {"idle", "done"}) ~> (run[r].pc = "done")
=============================================================================
