# Workspace Log Schema

This document describes the structure of the JSON Lines in `tests/SAMPLE.jsonl`, which capture Claude workspace interaction logs.

## Top-level fields
- **parentUuid** (`string | null`): Identifier of the message this entry responds to. `null` for root messages.
- **isSidechain** (`boolean`): Whether the message was generated on a sidechain. Observed as `false`.
- **userType** (`string`): Source of the message. Examples include `"external"` for user generated entries.
- **cwd** (`string`): Current working directory for the log entry.
- **sessionId** (`string`): Unique identifier for the conversation session.
- **version** (`string`): Client or protocol version string.
- **gitBranch** (`string`): Active git branch during logging.
- **type** (`string`): Message origin, e.g. `"user"` or `"assistant"`.
- **message** (`Message`): Nested object describing the actual message payload (see below).
- **uuid** (`string`): Unique identifier for this log entry.
- **timestamp** (`string`): ISO‑8601 timestamp.
- **requestId** (`string`, optional): Identifier for the API request, present on assistant responses.
- **toolUseResult** (`string | object`, optional): Raw output returned from a previous `tool_use` call.
- **isApiErrorMessage** (`boolean`, optional): Indicates the entry represents an API error message.

## Message object
Fields found inside `message` depend on the entry type:
- **role** (`string`): `"user"` or `"assistant"`.
- **content** (`string | Content[]`): Either a plain text string or an array of structured `Content` items.
- **id** (`string`, optional): Unique message identifier for assistant entries.
- **type** (`string`, optional): Typically `"message"` for assistant entries.
- **model** (`string`, optional): Name of the model generating the assistant response.
- **stop_reason** (`string | null`, optional): Reason generation ended.
- **stop_sequence** (`string | null`, optional): Stop sequence that triggered termination.
- **usage** (`Usage`, optional): Token and request accounting information (see below).

## Content items
When `message.content` is an array, each item has a `type` field determining its structure:

### TextContent
- `type`: `"text"`
- **text** (`string`): Textual content.

### ToolUseContent
- `type`: `"tool_use"`
- **id** (`string`): Tool invocation identifier.
- **name** (`string`): Name of the tool invoked.
- **input** (`object`): Parameters provided to the tool (schema depends on the tool).

### ToolResultContent
- `type`: `"tool_result"`
- **tool_use_id** (`string`): References the originating `tool_use` entry.
- **content** (`string | object`): Output returned by the tool.
- **is_error** (`boolean`, optional): Whether the tool reported an error.

## Usage object
Token accounting for assistant responses:
- **input_tokens** (`number`): Tokens in the request.
- **output_tokens** (`number`): Tokens generated in the response.
- **cache_creation_input_tokens** (`number`): Tokens stored in cache.
- **cache_read_input_tokens** (`number`): Tokens retrieved from cache.
- **service_tier** (`string | null`): Service tier used (e.g. `"standard"`).
- **cache_creation** (`CacheCreation`, optional): Details for ephemeral cache usage.
- **server_tool_use** (`ServerToolUse`, optional): Metrics for server-side tool usage.

### CacheCreation object
- **ephemeral_5m_input_tokens** (`number`): Tokens cached for ~5 minutes.
- **ephemeral_1h_input_tokens** (`number`): Tokens cached for ~1 hour.

### ServerToolUse object
- **web_search_requests** (`number`): Count of web search tool requests during generation.

---
This schema was derived from inspecting the 227 entries in `tests/SAMPLE.jsonl` and is intended to help interpret Claude workspace logs.
