# Project testing

Contains some notes for testing the project manually.

```bash
dog records-post.yaml | yq --output-format json > records-post.json
curl http://[::]:8888/records -H "Content-Type: application/json" --json @records-post.json
```

