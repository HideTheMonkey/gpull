# gpull (git pull)
`gpull` is a simple wrapper for `git pull`, with the addition that it also watches for any changes to `package.json` or `package-lock.json` and if detected will automatically run `npm i` afterwards.

_Now you don't have to worry about your dev environment malfunctioning when someone else makes updates!!_

## Build
`cargo build --release` will create `./target/release/gpull`

## Installation
I recommend putting the `gpull` binary somewhere on your local $PATH and creating an alias like `alias gpl='gpull'` so you can easily run `gpl` in whichever repo you want.
