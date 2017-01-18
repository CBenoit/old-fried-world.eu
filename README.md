Fried world website old source code
===================================

I made my website with Rust because I wanted to start using the
language on something concrete after reading the Book and various
articles.
I started it during november 2016.
To begin with, I looked for a good Rust web framework.
[Are We Web Yet](http://www.arewewebyet.org/) has been a good source of information about that.
I end up choosing Iron framework.

The thing is, I'm not really a web guy and Iron is not the worst about it but
I found the documentation quite short on a lot of things. I was often
wondering what it that thing? How do I use it? When is it useful?
Moreover, Iron aims to be very extensible and does not provides much out of the box.
But it's okay and I was aware of it. The thing is that I was a little bit lost at first:
which crate should I use?
Fortunately I could find some public repositories like the one I'm sharing today
to find some directions.
Hopefully It can be of help too.

By the time I write this README (on 18 January 2017), another web framework became a rising star and
that's [Rocket web framework](https://rocket.rs). And this time, the documentation is really good,
the API is very pleasant and focused on ease-of-use. Yeah, I'm definitely porting my website to use Rocket.

## About static files

For development purpose, I made a `serves_static` compilation feature for static files serving.
Simply run `cargo run --features serves_static` to have it enabled.
On my hosting machine there is a nginx server to perform reverse proxy and serves static file.
For production, I can then simply run `cargo run --release`.

