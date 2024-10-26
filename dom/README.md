# DOM

This project subcrate includes a basic generic DOM implementation, with ways to query the DOM in an safe way.

## DOM Queries

Dom queries are done like so :

```rust
let query = DOM!(div > h1 > p);
let [div, h1, p] = query.findResults(Documents)
```
