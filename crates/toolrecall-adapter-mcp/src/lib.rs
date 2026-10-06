use rmcp::model::Tool;
use serde_json::{Map, Value};
use toolrecall_domain::ToolDescriptor;

pub fn to_rmcp(tool: &ToolDescriptor) -> Result<Tool, String> {
    let obj: Map<String, Value> = tool
        .input_schema
        .as_object()
        .cloned()
        .ok_or("input_schema must be an object")?;
    Ok(Tool::new(tool.name.clone(), tool.description.clone(), obj))
}
pub fn validate_catalog(catalog: &[ToolDescriptor]) -> Result<usize, String> {
    catalog
        .iter()
        .map(to_rmcp)
        .collect::<Result<Vec<_>, _>>()
        .map(|x| x.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructs_real_rmcp_tool() {
        let d = ToolDescriptor {
            source: "crm".into(),
            name: "lookup".into(),
            description: "lookup customer".into(),
            input_schema: serde_json::json!({"type":"object","properties":{}}),
            deferred: true,
        };
        let t = to_rmcp(&d).unwrap();
        assert_eq!(t.name, "lookup");
    }
}
