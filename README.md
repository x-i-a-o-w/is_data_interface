# Data interface

_Cross-platform library for handling interfaces such as the filesystem._

## !!!Disclaimer!!!

(because of the not yet implemented read_ignore implementation that will be used to read a file while it is locked meanwhile it is being watched)

This library is still in experimental state.
Do not use this library for projects that are security critical and/or have a "busy" user system.

It is recommended to use this library for regular user space

STILL W.I.P.
I will set feature flags for this crate and tweak some interfaces because I made them more complex then they need to be

## Core Idea

Use the [InterfaceManager] trait to quickly turn a single read of [Interface] into a multi read which is flattened into the [VecResult].
The main point of this crate is to be easely modified.
Simular to std::io::{Read,Write} in function, but focused more on flexibility and scale rather then reading and/or writing a \[u8\] buffer.
Also code is quite simple so if you want to do anything with it do not be afraid.

Originally designed to work with only the filesystem, but it can be modified to use other sources.

## Goals

- Create a framework for building scalable and flexible interfaces
- Easy interface error handling and recovery
- Simplify watching multiple interfaces(files/paths)
- Provide an "reference" counted interface 
- Make interface operations ignorable(when under watcher)
- Provide basic implementations for interface management

## Platforms

Platforms are inherited from the [notify] crate.

## License

[notify]: https://github.com/notify-rs/notify/
[InterfaceManager] 
[VecResult]
