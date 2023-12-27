use actix_web::{
    get,
    web::Json
};
use serde::{Serialize};

#[derive(Serialize)]
pub struct Agent {
    name: String
}

#[derive(Serialize)]
pub struct AgentsResponse {
    count: usize,
    agents: Vec<Agent>
}

#[get("/api/v1/agents/")]
pub async fn get_agents() -> Json<AgentsResponse> {
    let agent1 = Agent{ name: "Test Agent".to_string() };
    let agent2 = Agent{ name: "Second Agent".to_string() };
    let agents = vec!(agent1, agent2);
    let resp = AgentsResponse{ count: agents.len(), agents };

    Json(resp)
}
