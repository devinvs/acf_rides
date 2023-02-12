# ACF Rides

A reminder that a tool that isn't as flexible as what it replaces will most
often fall into disuse.

In college I was on leadership of club which often was orchestrating rides from place to
place, usually multiple events a week with multiple drivers. Commonly this
process took place either in-person (and usually no one was left behind) or via
a discord chat. The process was chaotic but flexible.

In a meeting one of the leaders mentioned that their friend was part of a group
that used an app to arrange rides, which sounded like a good idea to us.

Naively, I said I could build such an app that would work well for us.

6 months after releasing this app (whose source is seen above) it has fallen
completely into disuse, and I'm going to be shutting it all down. As I do this I
wanted to write a quick retrospective on why I think it failed.

### 1. Logins/Accounts management

By requiring accounts for every person who wanted to use the app, every time
they wanted to use it required a login. This allowed us to keep track of who had
what cars and who was driving what, but it would have probably been better to
try to develop a per-device fingerprint of sorts. The login/account process adds
friction both to people using the app regularly and to people who only need a
ride once or twice.

See whentomeet for an alternative system with less friction.

### 2. Not Flexible Enough

This is partly because I wrote this very quickly as an experiment of building a
webapp in Rust, but the interactions were not flexible enough. Arranging rides
in person or over discord is very flexible, and I attempted to impose a strict
workflow on top of that process. 

For instance, editing of any resource doesn't currently exist except by
deleting and creating a new resource.

This made it harder for both drivers and riders to get what they actually
needed, which was a ride.

### 3. Not enough transparency

Another frustration I think for leadership was that they couldn't see who was
riding with who. There wasn't any central way to view the current state of the
system.

Without centralized control leadership was less inclined to use it because they
couldn't be sure it was working as expected.

### Is this it?

Maybe, maybe not. A lot of these flaws could still be fixed to create a superior
experience. I must be reminded to always, ALWAYS prioritize the experience over
the technology. I had that flipped for this project and it suffers for it.
