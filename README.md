# 0x0bot

A Discord bot designed around the [0x0b](https://discord.gg/Bqnc2cq) guild.

It is being developed and maintained by the 0x0b staff team.

## ❗ Disclaimer

This bot is neither intended to be public nor designed to run in other environments
than our own. It is heavily customized to the very specific needs of our Discord
guild. Thus, we won't assist with issues pertaining to setting up custom instances
of the bot. Source code is provided as-is for educational purposes and for people
who are interested in contributing.

## 📘 Commands

Coming soon.

## 💻 Setup

#### Rust

This bot requires **Rust >= 1.37**, make sure to
[install it from here](https://www.rust-lang.org/tools/install).

#### PostgreSQL

Further, **PostgreSQL** is needed. You may want to search for
instructions on how to install those for your platform separately.

Once this is done, a database must be created for the bot. It should preferably be
owned by a role with no dangerous privileges, that's why a new one will be created:

```sql
CREATE ROLE 0x0b LOGIN PASSWORD '123';

CREATE DATABASE 0x0bot OWNER 0x0b;
CREATE EXTENSION pg_trgm;
```

We need to construct a database URL for connecting to our newly created database.

Given these SQL queries, `user` (`0x0b`), `password` (`123`) and `database` (`0x0bot`)
for our database URL are known, i.e. `postgres://0x0b:123@host/0x0bot`. `host` has to
be provided by yourself.

At this point, choose whether you want 0x0bot to run inside Docker or not.

#### Without Docker

Copy the contents of [`.env.sample`](./.env.sample), paste them into a file
called `.env` which serves as the configuration for the bot and fill out all fields.

Finally you can build and run the bot. During the build process, database migrations
will be automatically applied and all dependencies will be installed so you won't have
to take care of anything else.

```shell script
# Build the bot.
cargo build --release

# Run it.
cargo run
```

#### With Docker

Obviously, Docker will be required,
[install it from here](https://docs.docker.com/v17.12/install/).

The next step is to build the image. You won't have to create a `.env`
file at this approach, the values of the variables will be passed as
build arguments directly.

```shell script
# Coming soon.
```

Now you can run the bot image:

```shell script
# Coming soon.
```

## 🖋 License

0x0bot is distributed under the terms of either the MIT license or the Apache
license (Version 2.0), at the user's choice.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
