This is an interactive test of a Rust(subset)-to-Javascript transpiler;

As both things are very much in-progress, to make things
easier for making changes, this example expects
the transpiler checked out side by side to it, so before going further,
please do clone it as well:

```
cd ..
git clone https://github.com/ayourtch/mojes
    # then come back here
cd mojes-sample
    # then you can build and run it:
cargo run
    # many warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/irontest`
🚀 Rust-to-JS Transpiler Server starting...
📊 Server running on http://localhost:3000
🔧 DOM API uses native JavaScript camelCase method names
🎯 Open browser developer tools to see console output
```

From there on, it is as written - fire up the browser pointing to  http://localhost:3000 and have fun (do not forget about dev tools and "view source" !
Evidently you will want to have another screen nearby with the "src/main.rs" opened as well.
