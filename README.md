# Simple RSS viewer

This is just a simple RSS application written to experiment with Actix framework and Sencha Touch (which is good enough to
work without any broken-by-design pieces of crap like nodejs, npm or hipster driven frameworks like react or angular).

It will probably never be improved so if you want to file an issue, you have to also attach the pull request.

## Build and run

1. Building backend is simple and straightforward if you have [Rust](https://www.rust-lang.org/) and Cargo.
Just `cargo run` would be enough to start the server.

If you also want to autoreload server on changes, use:
```
$ cargo install systemfd cargo-watch
$ systemfd --no-pid -s http::8080 -- cargo watch -x run
```

2. To build frontend counterpart you need to install [Sencha Touch](https://www.sencha.com/products/touch/).

If you are running server locally, just build it in the `rssfront` folder and serve with any static file server:
```
rssfront$ sencha app build
rssfront$ python3 -m http.server
```

Or if you want to put backend and frontend behind some proxy (e.g. nginx), you need to edit `rssfront/app/util/Config.js` to fix paths.
Then build it in production mode and serve:
```
rssfront$ sencha app build production
rssfront/build/production/rssfront$ python3 -m http.server
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you shall be licensed as above, without any additional terms or conditions.
