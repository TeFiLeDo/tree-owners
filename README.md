# tree-owners

Find all owners (user and group) inside a directory tree.

© Adrian Wannenmacher - Licensed under the EUPL

## Usage

To see available options run: `tree-owners --help`

Basic example when running in this repository:
```
$ tree-owners
users:
    adrian (1000)

groups:
    users (985)
```

Using `uid`s and `gid`s:
```
$ tree-owners --raw
users:
    1000

groups:
    985
```

Using `json` output:
```
$ tree-owners --json
{
  "users": {
    "1000": "adrian"
  },
  "groups": {
    "985": "users"
  }
}
```

Combining `json` and `uid`/`gid`:
```
$ tree-owners --raw --json
{
  "users": {
    "1000": null
  },
  "groups": {
    "985": "users"
  }
}
```
