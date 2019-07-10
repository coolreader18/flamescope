# flamescope

Export [flame](https://github.com/TyOverby/flame) data to
[speedscope](https://www.speedscope.app/)'s format.

## Usage

```rust
use flame;
use flamescope;
use std::fs::File;

fn main() {
    let main_guard = flame::start_guard("main");
    {
        let _scope_guard = flame::start_guard("inner scope");
    }
    main_guard.end();
    flamescope::dump(&mut File::create("flamescope.json").unwrap()).unwrap();
}
```

## License

This project is licensed under the MIT license. Please see the
[LICENSE](LICENSE) file for more details.
