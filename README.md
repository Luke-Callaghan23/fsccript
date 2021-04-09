# fscript
fscript is (or will be) a small transpiler for Javascript, written in rust that aims to make Javascript just a bit more like a Functional Programming Language (WIP)


Examples of this will be:

* Arrow functions will now be able to return objects: 
   * "const f = a => { a : a }" is not valid Javascript, but (will be) valid fscript
* If statements will be able to be used as expressions:
   * "const result = if(a == 'true') { true } else { false }" (will set) the value "result" to true or false, depending on whether a == 'true' or not.
* Switch statements will be able to be used as expressions:
   * "const retuls = switch(key) { case 'true': true; break; case 'false': false; break }" (will set) the value "result" to true or false, depending on the value of key.


There are more things that I may plan to add later on, but this is all that I am confident I'd be able to do now.  Rust is a difficult language.
