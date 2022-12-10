<p align="center">
  <img src="https://github.com/jjant/runty8/blob/26ae9f0bb6ae243afb263386c2ec81a0293b6983/img/logo.png" alt="Runty8 Logo" />
</p>

# runty8-game-template

Use this template to create your own [`runty8`](https://github.com/jjant/runty8) game.

## Running

Use `cargo run` to run your game natively.

Use `./build_script.sh runty8-game-template` (or your binary's name) to run your game in the browser.

This requires [`serve`](https://github.com/vercel/serve).

## Creating your own game

Head over to your [`src/main.rs`](./src/main.rs) to implement your own game.

Feel free to take a look at the code for our port of [Celeste](https://github.com/jjant/runty8/blob/425efffb2dd134eaf5f06d5e752642c6da9926c6/img/celeste.gif) for some guidance, [here](https://github.com/jjant/runty8/blob/425efffb2dd134eaf5f06d5e752642c6da9926c6/examples/celeste/main.rs).

Press the `Escape` key to switch between the game and the editor.

Press `Ctrl+S` to save changes to your assets (sprite sheet, sprite flags, etc).
