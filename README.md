# Rsip

Very flexible SIP parser.

## Features
* This thing is fast, uses nom for basic
  message parsing and headers are parsed only when needed, on-demand.
  Intentions are to make it even faster by providing a non-owning variant
  (everything is an &str underneath)
* Strong (new)types everywhere. Even if underlying type is String, everything is
  a NewType for better type safety.
* Provides typed headers on demand, like `From`, `To`, `Contact`, `Via` etc
  The reasoning behind on demand strongly typed headers is 2 fold:
  * perfromance & memory reasons: headers are parsed only when needed
  * it enables you to still have a working Rust SIP parser in case a typed header
  has a bug, the peer has a bug or there is an edge/new case never seen before.
* While performance is always a goal, user friendliness and usability is the main
 goal. A lot of helpful functions and convertions to make things easy :)
* Very simple code structure make it super easy to extend and add new typed headers
  As long as you can do [nom](https://github.com/Geal/nom) stuff, it's straightforward.
* Provides some extra services like Digest auth generator/validator etc
  Intention is to add many helper services.

Built for use in [viska](https://github.com/vasilakisfil/viska) SIP framework.
