//! MCP tool definitions for the scheduler extension.

use convergio_types::extension::McpToolDef;
use serde_json::json;

pub fn scheduler_tools() -> Vec<McpToolDef> {
    vec![
        McpToolDef {
            name: "cvg_scheduler_decide".into(),
            description: "Request a scheduling decision.".into(),
            method: "POST".into(),
            path: "/api/scheduler/decide".into(),
            input_schema: json!({"type": "object", "properties": {"task_id": {"type": "string"}, "constraints": {"type": "object"}}, "required": ["task_id"]}),
            min_ring: "trusted".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_scheduler_get_policy".into(),
            description: "Get current scheduling policy.".into(),
            method: "GET".into(),
            path: "/api/scheduler/policy".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_scheduler_history".into(),
            description: "Get scheduling decision history.".into(),
            method: "GET".into(),
            path: "/api/scheduler/history".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
    ]
}
