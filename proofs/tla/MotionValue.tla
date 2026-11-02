---------------------------- MODULE MotionValue ----------------------------
(***************************************************************************)
(* A model of `MotionValue` in crates/motion/src/value.rs: a number that   *)
(* one background loop moves a frame at a time, while event handlers may   *)
(* call set / stop / animate at any moment.                                *)
(*                                                                         *)
(* Dioxus runs tasks on one thread, so code only interleaves at `.await`.  *)
(* Each action below is one stretch of code between two awaits:            *)
(*   - Set, Stop, Animate: one call from an event handler.                 *)
(*   - Wake: one turn of the loop in `run()`, from the frame timer firing  *)
(*     to the next `MotionTime::delay(..).await` or to the loop's exit.    *)
(*   - Unmount: the owning component goes away.                            *)
(* Numbers are abstracted: a value is one of `Targets`, or "between" while *)
(* it is mid-flight; an animation is its target plus the frames it has     *)
(* left (springs and glides always settle; that is physics.rs's job, and   *)
(* is tested there).                                                       *)
(*                                                                         *)
(* The two Bug* constants switch on deliberate mistakes, so CI can show    *)
(* the checker really catches them (scripts/tla.sh).                       *)
(***************************************************************************)
EXTENDS Naturals

CONSTANTS
    Targets,         \* values a caller can ask for
    MaxFrames,       \* longest animation, in frames
    MaxActions,      \* calls a user may make (keeps the model finite)
    BugNoLoopGuard,  \* TRUE: `animate` spawns a loop even when one runs
    BugKeepLooping   \* TRUE: the loop exits on an empty slot without clearing `looping`

ASSUME MaxFrames \in Nat \ {0} /\ MaxActions \in Nat
ASSUME BugNoLoopGuard \in BOOLEAN /\ BugKeepLooping \in BOOLEAN

Positions == Targets \cup {"between"}

VARIABLES
    value,       \* the `value` signal
    animOn,      \* the `animation` slot is Some(..)
    animTarget,  \* ... heading for this target
    animLeft,    \* ... with this many frames left
    animating,   \* the `animating` signal
    looping,     \* the `looping` flag
    tasks,       \* loop tasks alive (each waiting at its `.await`)
    mounted,     \* the owning component is mounted
    reduced,     \* the reduced-motion setting
    goal,        \* ghost: where the value should come to rest
    actions      \* ghost: calls made so far

vars == <<value, animOn, animTarget, animLeft, animating, looping, tasks,
          mounted, reduced, goal, actions>>

Init ==
    /\ value \in Targets
    /\ animOn = FALSE
    /\ animTarget \in Targets
    /\ animLeft = 0
    /\ animating = FALSE
    /\ looping = FALSE
    /\ tasks = 0
    /\ mounted = TRUE
    /\ reduced \in BOOLEAN
    /\ goal = value
    /\ actions = 0

UserCall == mounted /\ actions < MaxActions

\* `stop()`: take the animation; clear `animating` only if there was one.
StopEffect ==
    /\ animOn' = FALSE
    /\ animLeft' = 0
    /\ animating' = IF animOn THEN FALSE ELSE animating

\* `set(x)`: stop, then jump.
Set(x) ==
    /\ UserCall
    /\ StopEffect
    /\ value' = x
    /\ goal' = x
    /\ actions' = actions + 1
    /\ UNCHANGED <<animTarget, looping, tasks, mounted, reduced>>

\* `stop()` from outside: rest where it is.
Stop ==
    /\ UserCall
    /\ StopEffect
    /\ goal' = value
    /\ actions' = actions + 1
    /\ UNCHANGED <<value, animTarget, looping, tasks, mounted, reduced>>

\* `animate(a)`: frames = 0 is an animation that is already done.
Animate(t, frames) ==
    /\ UserCall
    /\ actions' = actions + 1
    /\ goal' = t
    /\ IF reduced \/ frames = 0
          THEN \* reduced motion or done: `self.set(end)`
               /\ StopEffect
               /\ value' = t
               /\ UNCHANGED <<animTarget, looping, tasks, mounted, reduced>>
          ELSE /\ animOn' = TRUE
               /\ animTarget' = t
               /\ animLeft' = frames
               /\ animating' = TRUE
               /\ IF ~looping \/ BugNoLoopGuard
                     THEN /\ looping' = TRUE
                          /\ tasks' = tasks + 1   \* Runtime::spawn(run())
                     ELSE UNCHANGED <<looping, tasks>>
               /\ UNCHANGED <<value, mounted, reduced>>

\* The OS reduced-motion setting changes.
ToggleReduced ==
    /\ UserCall
    /\ reduced' = ~reduced
    /\ actions' = actions + 1
    /\ UNCHANGED <<value, animOn, animTarget, animLeft, animating, looping,
                   tasks, mounted, goal>>

Unmount ==
    /\ mounted
    /\ mounted' = FALSE
    /\ UNCHANGED <<value, animOn, animTarget, animLeft, animating, looping,
                   tasks, reduced, goal, actions>>

\* One turn of `run()`. `timerErr`: the platform has no frame timer.
Wake ==
    /\ tasks > 0
    /\ IF ~mounted
          THEN \* `animation.try_write()` fails: return without touching anything
               /\ tasks' = tasks - 1
               /\ UNCHANGED <<value, animOn, animTarget, animLeft, animating,
                              looping, mounted, reduced, goal, actions>>
          ELSE \E timerErr \in BOOLEAN :
               IF ~animOn
                  THEN \* slot is None: break, then clear `looping`
                       /\ tasks' = tasks - 1
                       /\ looping' = IF BugKeepLooping THEN looping ELSE FALSE
                       /\ UNCHANGED <<value, animOn, animTarget, animLeft,
                                      animating, mounted, reduced, goal, actions>>
                  ELSE IF timerErr \/ animLeft = 1
                  THEN \* finished: jump to the end, clear the slot and the flags
                       /\ animOn' = FALSE
                       /\ animLeft' = 0
                       /\ value' = animTarget
                       /\ animating' = FALSE
                       /\ looping' = FALSE
                       /\ tasks' = tasks - 1
                       /\ UNCHANGED <<animTarget, mounted, reduced, goal, actions>>
                  ELSE \* one frame closer; wait for the next
                       /\ animLeft' = animLeft - 1
                       /\ value' = "between"
                       /\ UNCHANGED <<animOn, animTarget, animating, looping,
                                      tasks, mounted, reduced, goal, actions>>

Next ==
    \/ \E x \in Targets : Set(x)
    \/ Stop
    \/ \E t \in Targets, f \in 0..MaxFrames : Animate(t, f)
    \/ ToggleReduced
    \/ Unmount
    \/ Wake

\* Frames keep coming: a waiting loop eventually runs.
Spec == Init /\ [][Next]_vars /\ WF_vars(Wake)

-----------------------------------------------------------------------------
(* What must hold *)

TypeOK ==
    /\ value \in Positions
    /\ animOn \in BOOLEAN /\ animTarget \in Targets
    /\ animLeft \in 0..MaxFrames
    /\ animating \in BOOLEAN /\ looping \in BOOLEAN
    /\ tasks \in 0..MaxActions
    /\ mounted \in BOOLEAN /\ reduced \in BOOLEAN
    /\ goal \in Positions /\ actions \in 0..MaxActions

\* Never two loops moving one value (they would each advance it a frame).
OneLoop == tasks <= 1

\* A running animation always has a loop to drive it, so it can't freeze.
NoFrozenAnimation == mounted /\ animOn => tasks >= 1

\* `is_animating()` tells the truth.
AnimatingIsTrue == mounted => (animating = animOn)

\* `looping` is set exactly while a loop is alive.
LoopingIsTrue == mounted => (looping = (tasks >= 1))

\* At rest, the value is where the last call asked it to be.
RestsAtGoal == mounted /\ ~animOn => value = goal

\* Nothing writes to a component that has gone.
NoWriteAfterUnmount == [][~mounted => UNCHANGED <<value, animating, looping>>]_vars

\* Every animation finishes (or its component goes away).
Settles == (mounted /\ animOn) ~> (~animOn \/ ~mounted)

\* ... and every loop task ends, so nothing keeps waking up for nothing.
LoopsEnd == (tasks > 0) ~> (tasks = 0)
=============================================================================
