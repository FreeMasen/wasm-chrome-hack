let js = import('./wasm_chrome_hack.ch.js');
js.then(mod => {
    mod.booted.then(() => {
        let p = mod.Person.new("Bilbo", 111);
        mod.say_hi(p);
    });
});