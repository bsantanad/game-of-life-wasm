# game of life w/ rust and webassembly

## acknowledgement

this project was done following this guide: [rust book][guide], also
it was done by using this template: [wasm-pack-template][temp]

the purpose was to learn webassembly and rust, how they interact together
and what can be done with it

## install

**IMPORTANT**: you'll need rust, cargo and npm installed in your machine 

1. clone the repo and `cd` to the dir:
```
git clone git@github.com:bsantanad/game-of-life-wasm.git game-of-life.git
cd game-of-life.git/
```
2. then build the project
```
cargo build
wasm-pack build
```
3. install npm dependencies
```
cd www/
npm i
```
4. start the server
```
npm run start
```

## usage
open a browser and go to `localhost:8080`

[guide]: https://rustwasm.github.io/book/introduction.html
[temp]: https://github.com/rustwasm/wasm-pack-template

