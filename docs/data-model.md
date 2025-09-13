
# Data Model (MongoDB)

## polls
```json
{
  "_id": "ObjectId",
  "slug": "string",
  "title": "string",
  "description": "string",
  "options": ["string"],
  "status": "draft|published|closed",
  "start": 1690000000,
  "end": 1690005000,
  "vote_price": "10000",
  "txhash": "string"
}
```

## votes_idx
```json
{
  "_id": "ObjectId",
  "poll_id": "string",
  "voter": "cosmos1...",
  "option_index": 0,
  "txhash": "string"
}
```

## results_cache
```json
{
  "_id": "ObjectId",
  "poll_id": "string",
  "counts": [12, 5, 7],
  "updated_at": "ISODate"
}
```
