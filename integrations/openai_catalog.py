"""Real OpenAI Agents SDK catalog boundary; no model call or API key required."""
import json
from typing import Annotated
from agents import function_tool

def descriptor(source, tool):
    return {"source": source, "name": tool.name, "description": tool.description,
            "input_schema": tool.params_json_schema, "deferred": bool(tool.defer_loading)}

@function_tool(defer_loading=True)
def get_customer_profile(customer_id: Annotated[str, "Customer identifier"]) -> str:
    """Fetch account identity and tier."""; return customer_id
@function_tool(defer_loading=True)
def list_open_orders(customer_id: Annotated[str, "Customer identifier"]) -> str:
    """List active customer orders."""; return customer_id
@function_tool(defer_loading=True)
def get_invoice_status(invoice_id: Annotated[str, "Invoice identifier"]) -> str:
    """Lookup invoice payment state."""; return invoice_id
@function_tool(defer_loading=True)
def issue_refund(invoice_id: Annotated[str, "Invoice identifier"]) -> str:
    """Issue approved invoice refund."""; return invoice_id
@function_tool(defer_loading=True)
def get_shipping_eta(tracking_number: Annotated[str, "Tracking number"]) -> str:
    """Estimate parcel arrival by tracking number."""; return tracking_number
@function_tool(defer_loading=True)
def get_credit_balance(customer_id: Annotated[str, "Customer identifier"]) -> str:
    """Shipping credit balance."""; return customer_id
@function_tool(defer_loading=True)
def revoke_session(session_id: Annotated[str, "Session identifier"]) -> str:
    """Revoke compromised session."""; return session_id
@function_tool(defer_loading=True)
def audit_login(user_id: Annotated[str, "User identifier"]) -> str:
    """Inspect login audit trail."""; return user_id

tools=[("crm",get_customer_profile),("crm",list_open_orders),("billing",get_invoice_status),("billing",issue_refund),("shipping",get_shipping_eta),("shipping",get_credit_balance),("security",revoke_session),("security",audit_login)]
print(json.dumps([descriptor(source,tool) for source,tool in tools]))
