# Dynamic Renderer Usage

Use this guide instead of searching unrelated Worka checkouts or Cargo caches.

The app crate already depends on the `worka` facade. Inline elicitation and notification UI should use `worka::dynamic` only.

Minimal imports:

```rust
use serde_json::json;
use worka::dynamic::{
    A2uiBasicDocumentOptions, DynamicActionOwner, DynamicActionOwnerKind,
    DynamicEvent, DynamicNode, DynamicNodeKind, DynamicRenderEnv,
    DynamicViewDocument, DynamicViewOrigin, DynamicViewOriginKind,
    a2ui_basic_document, render_fission,
};
```

Minimal render pattern:

```rust
fn render_attention_preview() -> Widget {
    let root = DynamicNode::new("attention-root", DynamicNodeKind::Card)
        .child(DynamicNode::new("attention-title", DynamicNodeKind::Heading)
            .prop("label", json!("Approval needed")))
        .child(DynamicNode::new("attention-body", DynamicNodeKind::Text)
            .prop("label", json!("Review this item before the agent proceeds.")))
        .child(DynamicNode::new("attention-submit", DynamicNodeKind::Button)
            .prop("label", json!("Approve"))
            .action(DynamicEvent::Press, worka::dynamic::DynamicAction::Submit {
                payload: json!({"decision": "approved"}),
            }));
    let doc = DynamicViewDocument::new(
        "attention-preview",
        DynamicViewOrigin {
            kind: DynamicViewOriginKind::McpElicitation,
            source_id: "generated-app".into(),
            request_id: "local-preview".into(),
        },
        root,
    );
    match render_fission(
        &doc,
        &DynamicRenderEnv {
            locale: "en-GB".into(),
            theme: "worka".into(),
            disabled: false,
        },
        DynamicActionOwner {
            owner_kind: DynamicActionOwnerKind::McpElicitation,
            owner_id: doc.view_id.clone(),
        },
    ) {
        Ok(rendered) => rendered.widget,
        Err(_) => Container::new(Text::new("Approval needed")).padding_all(12.0).into(),
    }
}
```

If `docs/specification.json` declares `inline_attention[]`, the app source must import and dispatch `ATTENTION_LIST_JOB`, `ATTENTION_FETCH_DYNAMIC_VIEW_JOB`, and `ATTENTION_RESPOND_JOB` from `worka::broker::fission`, then render fetched dynamic documents with `render_fission`.

When an LLM/tool produces dynamic UI, it must produce A2UI Basic JSON, not a raw `DynamicViewDocument`. Convert it mechanically before rendering:

```rust
fn render_a2ui_basic(value: &serde_json::Value) -> Widget {
    let doc = match a2ui_basic_document(
        value,
        A2uiBasicDocumentOptions::new("generated-app", "dynamic-ui-request")
            .fallback_view_id("dynamic-ui"),
    ) {
        Ok(doc) => doc,
        Err(_) => return Container::new(Text::new("Unable to render requested action"))
            .padding_all(12.0)
            .into(),
    };
    match render_fission(
        &doc,
        &DynamicRenderEnv {
            locale: "en-GB".into(),
            theme: "worka".into(),
            disabled: false,
        },
        DynamicActionOwner {
            owner_kind: DynamicActionOwnerKind::A2ui,
            owner_id: doc.view_id.clone(),
        },
    ) {
        Ok(rendered) => rendered.widget,
        Err(_) => Container::new(Text::new("Unable to render requested action")).padding_all(12.0).into(),
    }
}
```
