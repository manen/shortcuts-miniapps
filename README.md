# `shortcuts-miniapps`

write logic for your shortcuts in rust

## how it works

there's an executor shortcut on my phone, which when started, connects via ssh to the [`host`](host/src/main.rs). \
the host replies with a json blob containing actions, all of which are names of shortcuts executor modules with data in a format the specific module expects.
the big executor shortcut calls each module one after another.

the thing is the executor itself is a module too so your rust app can show you shortcuts uis indefinitely and call [any shortcut there's a module for](modules/all/src/lib.rs).

currently, communication is one way, so only the host can tell the phone things, but in theory it should work the other way too soon using stdin. \
this will make it possible to work with the results of any shortcuts action, like listing your notes and calendar events.

## executor shortcut

yeah so they haven't invented git for ios shortcuts yet so i can't really share a copy of the executor and the modules i put together but if you're interested for whatever reason i'm sure we can work something out hmu on [ig](https://instagram.com/bercel.lol)
