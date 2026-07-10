# aotyapi

This is a simple axum based aoty fetcher that uses chromiumoxide in order to fetch a very basic and primative API to use sparsely when in need to get decent metadata.
You are rate limited every 2 minutes. I don't use this wretched website but I still don't want to be IP banned and it should be atleast at the rate that you would manually fetch information.
It is intended to be NOT used when you are NOT on a private tracker or non legal sources where you do NOT use the metadata tags and descriptors.
There are 3 simple routes; you have to replace your spaces with + instead FYI. This also does not support romanji or whatever; you'd have to use the unicode for international languages.

## routes

| GET | `/api/album/:slug` | album detail (scores, tracks, tags, cover) |
| GET | `/api/artist/:slug` | artist discography (all releases) |
| GET | `/api/list/:query` | search results (albums + matching artists) |

## response format

The errors follow `(StatusCode, Json<Value>)`:

```json
{ "error": "cooldown bud" }
```

Specfic success bodies are serialized structs. Just check the `entry`, `artistentry`, and `search` modules for field definitions.
