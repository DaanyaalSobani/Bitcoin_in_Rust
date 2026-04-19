curl -X POST https://api.codapi.org/v1/exec \
-H "Content-Type: application/json" \
-d '{
  "sandbox": "rust",
  "command": "run",
  "files": {
    "main.rs": "fn main() { println!(\"Direct API test successful!\"); }"
  }
}'