---------------------------- MODULE ChatFollowJump ----------------------------
(***************************************************************************)
(* ChatFollow.tla with "Jump to latest" clicked once, on a log of up to 2 *)
(* messages: a separate config so CI checks the follow logic at 3          *)
(* messages and the jump at 2 in minutes, not hours.                       *)
(***************************************************************************)
EXTENDS ChatFollow
=============================================================================
