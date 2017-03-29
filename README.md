# `index_queue`

[![Documentation](https://docs.rs/index_queue/badge.svg)](https://docs.rs/index_queue)
[![Crates.io](https://img.shields.io/crates/v/index_queue.svg)](https://crates.io/crates/index_queue)
[![Travis CI Build Status](https://travis-ci.org/Rufflewind/index_queue.svg?branch=master)](https://travis-ci.org/Rufflewind/index_queue)

A queue for unique indices (integers) with O(1) push/pop and O(1) lookup/removal.  It is a doubly-linked list with all its nodes stored inside a single Vec.  The queue is most memory efficient when the integers are relatively small and densely packed.  The implementation is similar to [`ixlist`](https://github.com/bluss/ixlist), but `index_queue` is more specialized: it allows querying whether an index already exists as well as removal by index, but does not allow duplicate indices.

The queue works well with indices obtained from array-based allocators such as [`vec_arena`](https://crates.io/crates/vec-arena) or [`slab`](https://crates.io/crates/slab).

This crate was originally created to implement a cooperative FIFO task scheduler ([`synchrotron`](https://github.com/Rufflewind/synchrotron)).
