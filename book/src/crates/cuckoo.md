# Cuckoo

Cuckoo is the modeling tool.
I built it because I got tired of trying to create a model in Rust code.
It's barely usable, and requires editing the source code to change models.
I pretty much hate everything about it, but it let's me turn pictures into JSON.

I wrote most of cuckoo over a week or ten days.
It is a [React](https://reactjs.org) app that uses [Redux](https://redux.js.org) as it's store.
Using React was a terrible idea.
It is totally __not__ the right framework for building a drawing tool.

## Controls

I'm including this to just show how brutish this tool is.
That said, I'm rather pleased with the group select/move functionality.
I'd have liked copy/paste, which technically wouldn't be that difficult, but I can't stand the idea of spending another minute writing TypeScript.
I'm also really happy that I have undo/redo.

One will note that the keyboard is limited to modifier keys.
This is because that's the information that comes with [ `MouseEvent` ](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent).
Adding a key listener, and making that work with the way drawing is implemented would have been a nightmare.

| Click Location | Modifier         | Effect                 |
| -------------- | ---------------- | -----------------------|
| Paper          | ⌘ + drag         | New Object             |
| Object         | ⌘ + drag         | New Relationship       |
| Object         | double-click     | Object Editor          |
| Object         | ⌥ + click        | Object Editor          |
| Object         | ⌃ + click        | Delete Object          |
| Relationship   | double-click     | Relationship Editor    |
| Relationship   | ⌥ + click        | Relationship Editor    |
| Relationship   | ⌃ + click        | Delete Relationship    |
| Paper          | ⌥ + click        | Undo Action            |
| Paper          | shift + click    | Redo Action            |
| Paper          | ⌃ + click        | Group Selection Toggle |
| Selected Group | drag             | Move Selected Group    |
| Paper          | double-click     | Reload Model from Disk |