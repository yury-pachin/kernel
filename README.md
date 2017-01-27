The Kernel
==========

TL;DR Autobalancing low-latency non-blocking zero-copy CAS-multicursor queues with priority tasks and scalable timers.

[![Build Status](https://travis-ci.org/AlgoTradingHub/kernel.svg?branch=master)](https://travis-ci.org/AlgoTradingHub/kernel)
[![Gitter Chat](https://img.shields.io/gitter/room/badges/shields.svg)](https://gitter.im/voxoz/kernel)
[![Crates IO](https://img.shields.io/crates/d/kernel.svg)](https://crates.io/crates/kernel)
[![Crates IO](https://img.shields.io/crates/v/kernel.svg)](https://crates.io/crates/kernel)

Features
--------

* MIO Compatible Network Server with Connections
* Vectorizable SMP-aware stream combinators
* MPSC, SPMC, SPSC queues with CAS-semantics on Ring Buffers
* Zero-Copy Interpreter and Queues
* Session Types and π-calculus semantics
* 5-20ns latency
* BERT protocol for VM stats
* AVX Vectorization
* Dedicated InterCore Bus Protocol (Star Topology)

Test The O Language
-------------------

```
$ cargo build ; rlwrap ./target/debug/o -init etc/init.q
   Compiling kernel v1.1.0 (file:///Users/maxim/depot/voxoz/kernel)
    Finished dev [unoptimized + debuginfo] target(s) in 8.1 secs
AP core 3
AP core 2
AP core 1
BSP core 0
Welcome to O language interpreter 1.1.0
o) fac:{$[x=1;1;x*fac[x-1]]};fac[20]
2432902008176640000
o)
```

Enable AVX Vectorization
------------------------

```
$ cat ./cargo/config

[target.x86_64-unknown-linux-gnu]
rustflags="-C target-feature=+avx,+avx2"

$ cargo build --release

$ objdump ./target/release/o -d | grep mulpd
   223f1:	c5 f5 59 0c d3       	vmulpd (%rbx,%rdx,8),%ymm1,%ymm1
   223f6:	c5 dd 59 64 d3 20    	vmulpd 0x20(%rbx,%rdx,8),%ymm4,%ymm4
   22416:	c5 f5 59 4c d3 40    	vmulpd 0x40(%rbx,%rdx,8),%ymm1,%ymm1
   2241c:	c5 dd 59 64 d3 60    	vmulpd 0x60(%rbx,%rdx,8),%ymm4,%ymm4
   2264d:	c5 f5 59 0c d3       	vmulpd (%rbx,%rdx,8),%ymm1,%ymm1
   22652:	c5 e5 59 5c d3 20    	vmulpd 0x20(%rbx,%rdx,8),%ymm3,%ymm3
$ objdump ./target/release/o -d | grep vpmul
   2251c:	c5 d5 f4 fb          	vpmuludq %ymm3,%ymm5,%ymm7
   22525:	c4 41 55 f4 c0       	vpmuludq %ymm8,%ymm5,%ymm8
   2253a:	c5 d5 f4 db          	vpmuludq %ymm3,%ymm5,%ymm3
   22547:	c5 cd f4 ec          	vpmuludq %ymm4,%ymm6,%ymm5
   22550:	c5 cd f4 ff          	vpmuludq %ymm7,%ymm6,%ymm7
   22562:	c5 cd f4 e4          	vpmuludq %ymm4,%ymm6,%ymm4
   22595:	c5 d5 f4 fb          	vpmuludq %ymm3,%ymm5,%ymm7
```

Sample
------

```rust
extern crate kernel;

use kernel::io::poll::*;
use kernel::io::tcp::*;
use kernel::io::server::*;
use kernel::io::console::*;

fn main() {
    let x = std::thread::spawn(|| net());
    let y = std::thread::spawn(|| console());
    x.join();
}

fn net() {
    let addr = "127.0.0.1:8000".parse::<std::net::SocketAddr>().ok().expect("Parser Error");
    let sock = TcpListener::bind(&addr).ok().expect("Failed to bind address");
    let mut poll = Poll::new().expect("Failed to create Poll");
    let mut net = Server::new(sock);
    net.run(&mut poll).expect("Failed to run server");
}

fn console() {
    let mut poll = Poll::new().expect("Failed to create Poll");
    let mut con = Console::new();
    con.run(&mut poll);
}
```

Test WebSocket Server
-------------------

Prerequisites:

```
$ sudo apt-get install webfs
$ brew install webfs
$ cd /usr/ports/www/webfs/ && make install clean
```

Open Browser:

```
$ open http://127.0.0.1:8001/etc/status/index.htm
```

Reading
-------

* Kohei Honda Session Types and π-calculus http://mrg.doc.ic.ac.uk/kohei/
* Rust version http://munksgaard.me/laumann-munksgaard-larsen.pdf
* Haskell Version http://users.eecs.northwestern.edu/~jesse/pubs/haskell-session-types/session08.pdf

Credits
-------

* Viktor Sovietov, Core Suggestions
* Anton Kundenko, Stream Processing
* Ievgenii Lysiuchenko, Optimizations, System Programming
* Denis Golovan, Vectorization, KCell, Interpreter
* Mykola Oleksiienko, K expertise
* Maxim Sokhatsky, General View

Inspiration
-----------
* Ken Pratt, Rusty Scheme
* Carl Lerche, MIO

