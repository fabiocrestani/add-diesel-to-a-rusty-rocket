# rocket-diesel-simple-example

## Preparation
`rustup default nightly`

`cargo install diesel_cli --no-default-features --features sqlite`

## References
* https://github.com/sean3z/rocket-diesel-rest-api-example
* https://rocket.rs/v0.4/guide/requests/
* https://rocket.rs/v0.4/guide/responses/

## TODO
* Use diesel migrations instead of having the .db file in the repo

## Remark
* This commit includes an experiment with multiple concurent accesses to the database
  - The objetive is to get rid of this error:
  ~~~
    thread 'Opening the 152. of Beer { id: Some(1), name: "Augustiner", style: "Hell", abv: 5.7 }
    <unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: DatabaseError(__Unknown, "database is locked")', src\beer.rs:36:14
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ~~~

## Hints
`$Env:ROCKET_CLI_COLORS=off`