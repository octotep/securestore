securestore
===========

`securestore` is a Rust library which is intended to provide a simple and easy
to use interface for storing sensitive information, like passwords. My ultimate
goal is to create a cross-platform password manager with this library. I want
to keep the database format simple to make porting this library to other
languages and platforms as simple as possible. As of such, data is stored as a
tree in the filesystem, and serialized to json, to make it easy to parse.

The only non-Rust dependency that this library should depend upon is
[`libsodium`](https://github.com/jedisct1/libsodium), in the form of
[`sodiumoxide`](https://github.com/dnaq/sodiumoxide).
