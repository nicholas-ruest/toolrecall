use std::collections::BTreeSet;
use toolrecall_domain::{QueryCase, ToolDescriptor};

fn t(source: &str, name: &str, description: &str) -> ToolDescriptor {
    ToolDescriptor {
        source: source.into(),
        name: name.into(),
        description: description.into(),
        input_schema: serde_json::json!({"type":"object","properties":{}}),
        deferred: true,
    }
}
pub fn catalog() -> Vec<ToolDescriptor> {
    vec![
        t(
            "crm",
            "get_customer_profile",
            "fetch account identity and tier",
        ),
        t("crm", "list_open_orders", "list active customer orders"),
        t(
            "billing",
            "get_invoice_status",
            "lookup invoice payment state",
        ),
        t("billing", "issue_refund", "issue approved invoice refund"),
        t(
            "shipping",
            "get_shipping_eta",
            "estimate parcel arrival by tracking number",
        ),
        t("shipping", "get_credit_balance", "shipping credit balance"),
        t("security", "revoke_session", "revoke compromised session"),
        t("security", "audit_login", "inspect login audit trail"),
    ]
}
fn q(id: &str, text: &str, source: &str, tool: &str) -> QueryCase {
    QueryCase {
        id: id.into(),
        text: text.into(),
        required_sources: BTreeSet::from([source.into()]),
        required_tools: BTreeSet::from([tool.into()]),
    }
}
pub fn cases() -> Vec<QueryCase> {
    vec![
        q(
            "q1",
            "what tier is this customer",
            "crm",
            "get_customer_profile",
        ),
        q(
            "q2",
            "has invoice 77 been paid",
            "billing",
            "get_invoice_status",
        ),
        q(
            "q3",
            "when does tracking ZX arrive",
            "shipping",
            "get_shipping_eta",
        ),
        q(
            "q4",
            "show suspicious authentication history",
            "security",
            "audit_login",
        ),
        q(
            "q5",
            "customer asks whether an order has shipped",
            "crm",
            "list_open_orders",
        ),
        q(
            "q6",
            "credit for late parcel",
            "shipping",
            "get_credit_balance",
        ),
    ]
}
