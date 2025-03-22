# invincible-reviver

Preserves custom invincible mobs in Minecraft Bedrock worlds.

There is an unintended feature in Minecraft Bedrock that allows for entities to be both dead and alive at the same time, much like that of [Schrödinger's cat 🐱](https://www.google.com/search?q=schr%C3%B6dinger%E2%80%99s+cat).

This has been used by players to allow them to keep [entities alive even when they may be low on health](https://www.youtube.com/shorts/IhIuUh2a-wI), or a variety of other reasons. It essentially allows them to never take damage or to be killed by commands because the game already thinks they are dead, yet their data is still retained in the world.

Mojang is looking to [patch this feature](https://bugs.mojang.com/browse/MCPE/issues/MCPE-190466), and in doing so, they've decided to [remove these entities](https://www.reddit.com/r/PhoenixSC/comments/1j4aboo/plz_help_us/) in limbo, rather than allowing the player to decide what to do with them.

PhoenixSC made a nice video demonstrating more information about this, [check it out here](https://www.youtube.com/watch?v=J7s_jBRU5bI).

This tool is designed to bring these entities back to life again, out of that unintended limbo state. Rather than losing them altogether, you can keep them into future updates, where Mojang won't allow them to stay around.

## Usage

This is a command line program which is run through the terminal.
- On Windows, you will want to use PowerShell. You don't want to use the older Command Prompt.

### Modes

There are two modes to run this with, either to print invincible entities in your world, or to revive them "back to life".

```sh
./invincible-reviver <path-to-your-world> --print

# or

./invincible-reviver <path-to-your-world> --revive
```

### Print

For the print mode, it doesn't make any modifications to the world itself, it only traverses the database to find entities that are "invincible".

```sh
./invincible-reviver "./my fancy survival world/" --print
```

### Revive

For the revive mode, it will update the entity's NBT data to reset it back from it's `Dead` state, effectively bringing it back to life again. This changes the original data in the world, so the program automatically will make a backup for you before running this. If you are someone who likes living on the edge, there is an optional `--no-backup` flag you can pass to the end of the command for revive mode, and it will skip making the `.mcworld` archive.

```sh
./invincible-reviver "./my fancy survival world/" --revive
```
