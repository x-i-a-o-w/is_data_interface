# Data interface

_Cross-platform library for handling interfaces such as the filesystem._

## !!!Disclaimer!!!

Do not use this library for projects that are security critical and/or have an user system.
This library is still in experimental state.

It is recommended to use this library for regular user behavior 

## Goals

- Create a framework for building scalable interfaces
- Easy interface error handling
- Simplify watching multiple interfaces(files/paths)
- Provide an "reference" counted interface 
- Make interface operations ignorable(when under watcher)
- Provide basic implementations for interface management

## Platforms

Platforms are inherited from the [notify] crate.

## License

[notify]: https://github.com/notify-rs/notify/
