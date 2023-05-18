
Code in this crate MUST be platform agnostic and is expected to be able to run in any platform: bare-metal, desktop, mobile.

Because of that it MUST NOT depend upon rust `std` lib and must be [`no_std` combatible](https://docs.rust-embedded.org/book/intro/no-std.html).

Except the unit tests, which MAY depend on `std` if necessary.

> NOTE: Capitalized words are being used as defined in the [RFC2119](https://datatracker.ietf.org/doc/html/rfc2119) spec. 