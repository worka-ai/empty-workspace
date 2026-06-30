# Specification - Empty Workspace

The app runs as a Worka-managed native app/plugin and uses the broker client for all backend access.
It must not embed production credentials or mock business data. All records come from PersonalDB or
brokered MCP pack calls.

The generated app MCP server is the agent control surface for the workspace domain. Agent writes go
through those tools, which call domain services and mutate PersonalDB.
