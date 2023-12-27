use actix_web::{
    get,
    web::Json,
    web::Path,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AgentIdentifier {
    agent_id: String
}

#[get("/admin/agent/{agent_id}")]
pub async fn get_agent(agent_identifier: Path<AgentIdentifier>) -> Json<AgentIdentifier> {
    Json(agent_identifier.into_inner())
    /*Json(format!("not implemented: {}", agent_identifier.agent_id).to_string())*/
}
