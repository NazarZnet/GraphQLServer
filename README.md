# GraphQL Server
Written with educational purpose

## Getting started
Make sure you have installed [PostgreSQL](https://www.postgresql.org) and [Rust](https://www.rust-lang.org/).
```
git clone https://github.com/NazarZnet/GraphQLServer
cd GraphQLServer
cargo run
```

Configure **`.env`** file for your database environment!
Open http://127.0.0.1:8080/graphiql

## Key Technologies:
- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [Juniper](https://crates.io/crates/juniper)
- [Sqlx](https://crates.io/crates/sqlx)
- [PostgreSQL](https://www.postgresql.org/)

## API Documentation

|            **URI**            | **METHOD** |                                          **DESCRIPTION**                                               |
|:-----------------------------:|:----------:|:-----------------------------------------------------------------------------------------------------------:|             
|          /signup         |    POST    | Register new user. Send `name`, `username`, and `password` in JSON format. Returns created user                   |
|          /login          |    POST    | User log in. Send `username` and `password` in JSON format. Returns operation status and message    |
|          /graphiql            |     GET    | GraphQl playgroung
|         /graphql         |     POST    | Send your queries and mutations here                     |

### GraphQl key fucntions:
| Function                   | Description                                                           |
|----------------------------|-----------------------------------------------------------------------|
| version                    | Returns server version                                                |
| users                      | Returns list off all users with fields: `id`,`name`,`friends`                |
| user(name:String!)         | Returns information about specific user by name                       |
| aboutMe                   | Returns detail information, like: `id`, `name`, `username`,`loggedAt`,`friends` |
| addFriend(name:String!)    | Add friend to the user by name                                        |
| deleteFriend(name:String!) | Delete friend by name                                                 |