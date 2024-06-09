use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Degraded Performance")]
    DegradedPerformance,
    #[serde(rename = "Operational")]
    #[default]
    Operational,
    #[serde(rename = "Partial Service Disruption")]
    PartialServiceDisruption,
    #[serde(rename = "Planned Maintenance")]
    PlannedMaintenance,
    #[serde(rename = "Service Disruption")]
    ServiceDisruption,
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::DegradedPerformance => write!(formatter, "⚠️ Degraded Performance"),
            Status::Operational => write!(formatter, "✅ *Operational*"),
            Status::PartialServiceDisruption => {
                write!(formatter, "🔧 *Partial Service Degradation*")
            }
            Status::PlannedMaintenance => write!(formatter, "👷‍♂️ *Planned Maintenance*"),
            Status::ServiceDisruption => write!(formatter, "🚨 *Service Disruption*"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum State {
    #[default]
    Identified,
    Investigating,
    Monitoring,
    Resolved,
}

impl fmt::Display for State {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Identified => write!(formatter, "🔍 *Identified*"),
            State::Investigating => write!(formatter, "🕵️‍♂️ *Investigating*"),
            State::Monitoring => write!(formatter, "👀 *Monitoring*"),
            State::Resolved => write!(formatter, "✅ *Resolved*"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusIOWebhook {
    pub id: String,
    #[serde(rename = "message_id")]
    pub message_id: String,
    pub title: String,
    pub datetime: String,
    #[serde(rename = "current_status")]
    pub current_status: Status,
    #[serde(rename = "current_state")]
    pub current_state: State,
    #[serde(rename = "previous_status")]
    pub previous_status: Status,
    #[serde(rename = "previous_state")]
    pub previous_state: State,
    #[serde(rename = "infrastructure_affected")]
    pub infrastructure_affected: Vec<InfrastructureAffected>,
    pub components: Vec<Component>,
    pub containers: Vec<Container>,
    pub details: String,
    #[serde(rename = "incident_url")]
    pub incident_url: String,
    #[serde(rename = "status_page_url")]
    pub status_page_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfrastructureAffected {
    pub component: String,
    pub container: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub name: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub name: String,
    #[serde(rename = "_id")]
    pub id: String,
}
