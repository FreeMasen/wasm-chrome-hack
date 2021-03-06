# WASM Bindgen Chrome Hack

This project is designed to automate the process of importing a wasm module over 4k when using [WASM bindgen](https://github.com/rustwasm/wasm-bindgen), Webpack and Chrome. 

## Terminal Usage

To use this tool need to provide the path your your wasm-bindgen generated .js file. This is the file that has a bunch of js functions that start with `__wbg`, it should have the same name as your original wasm file but with a .js extention. Wasm-bindgen will install this file next your your module_bg.wasm file. Optionally you can provide an output path (this must include the name of your js file), if this paramter is not provided the file will be stored in the current directory with the same name as your wasm module with the extention `.ch.js`.

> Example
```sh
$ wbch ./wasm.js ./wasm.ch.js
```

## Output Usage

To use the `.ch.js` file that was generated by the tool, would need to include the following in your index.js file.

```js
//index.js
let js = import('./wasm_chrome_hack.ch.js');

js.then(mod => {
    mod.booted.then(() => {
        //do stuff with your wasm module
    });
});
```
If you have followed along with the hello world example in the wasm-bindgen repo, you will notice there is an additiona promise we need account for. First, we need to use the `import()` function to get the js file, once that is successfully loaded, we then need to wait for the wasm file to download, this is captured in the `.booted` property, once this has completed all of the module's properties will be available. Now you can run your `webpack-dev-server` and you should see everything working, even on chrome.

## Downloading
Current this isn't hosted anywhere outside of this repo. From what I understand the webpack implementation of the wasm import is considered a bug and should be fixed soonish so it seems pointless to take another name from cargo.

That means you will need to manually download the repo and build it yourself. 

```sh
$ git clone https://github.com/FreeMasen/wasm-chrome-hack
$ cd ./wasm-chrome-hack
$ cargo build
$ ./target/debug/wbch ./infile.js
```
## Why?
Currently wasm-bindgen offers the command `wasm2es6js` to convert your `.wasm` file into a `.js` file that includes the module as a base64 encoded string. This is a great work around, however if you build the project in the `test` folder and use this method you add about 100kb to the total size (in debug mode). To see for yourself you could do the following.

```sh
$ cd test
$ cargo +nightly build --target wasm32-unknown-unknown
$ wasm-bindgen ./target/wasm32-unknown-unknown/debug/wasm_chrome_hack.wasm
$ wasm2es6js --base64 -o ./wasm_chrome_hack_es.js ./wasm_chrome_hack_bg.wasm
```

For my system the .wasm file is ~275kb while the es6js file is ~370kb