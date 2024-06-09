use crate::bot;
use crate::status_io;
use crate::status_io::StatusIOWebhook;

use axum::{http::StatusCode, Json};

pub async fn handle_notification(
    Json(status): Json<status_io::StatusIOWebhook>,
) -> (StatusCode, &'static str) {
    print!("{:?}", status);

    bot::notify(build_message(status)).await;

    (StatusCode::ACCEPTED, "SENT !")
}

fn get_component_from_status(status: StatusIOWebhook) -> String {
    let component_id: String = status.infrastructure_affected[0].component.clone();

    return status
        .components
        .into_iter()
        .find(|comp| comp.id == component_id)
        .unwrap()
        .name;
}

fn get_container_from_status(status: StatusIOWebhook) -> String {
    let container_id: String = status.infrastructure_affected[0].container.clone();

    return status
        .containers
        .into_iter()
        .find(|comp| comp.id == container_id)
        .unwrap()
        .name;
}

fn escape_markdown_v2(text: String) -> String {
    return text
        .replace("!", "\\!")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("-", "\\-");
}

fn build_message(status: status_io::StatusIOWebhook) -> String {
    let message = format!(
        "{} - {}
📢 *{}*

📝 {}

🛠️ {} ({})",
        status.current_status,
        status.current_state,
        status.title,
        status.details,
        get_component_from_status(status.clone()),
        get_container_from_status(status.clone())
    );

    return escape_markdown_v2(String::from(message));
}
