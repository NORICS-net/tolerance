### Serde

Serializes into something like:
```
"width": {
  "value": 100000,
  "plus": 1000,
  "minus": -1000
}
```

While deserializing the `plus`- and `minus`-parts are optional (defaulting to ZERO).
The properties can be abbreviated to `v`, `p` and `m`. A json of `"width":{"v":10000}` would be valid.
