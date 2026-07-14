# Generated app MCP server template

Use this template only when `docs/specification.json` declares
`generated_app_mcp_server.required = true`.

Rules:

- Create the crate at the exact `generated_app_mcp_server.crate_path` value.
- Expose every `generated_app_mcp_server.tools[]` entry as an rmcp tool.
- Use `rmcp::transport::worka::WorkaTransport` and `WORKA_BROKER_SOCKET`.
- Validate typed request structs.
- Call the app domain service layer for every mutation.
- If `personaldb_groups[]` is non-empty, the domain service must use
  brokered PersonalDB only. The portable plugin-safe pattern is to return
  broker-applied `personaldb_operations` in the rmcp JSON payload so Worka can
  validate and apply them; direct agent writes are not allowed.
- Return serialized rmcp payloads with either `Content::text(body)` or
  `CallToolResult::structured(body_json)`.
- Emit or return audit evidence matching `activity_events[]`.
- Do not create fake tools, local mock backends, direct platform API calls, or
  UI-automation endpoints.
