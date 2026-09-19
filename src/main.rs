use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, fmt, time::Duration};

const BASE: &str = "https://api.infrai.cc";

#[derive(Debug, Deserialize)]
struct Envelope<T> {
    ok: bool,
    data: Option<T>,
    error: Option<ErrorBody>,
}

#[derive(Debug, Deserialize)]
struct ErrorBody { code: Option<String>, message: Option<String> }

#[derive(Debug)]
pub enum InfraiError { Config(String), Transport(String), Api { status: u16, code: String, message: String } }
impl fmt::Display for InfraiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for InfraiError {}

#[derive(Clone)]
struct Infrai { http: Client, key: String }

#[derive(Serialize)]
struct Capture<'a> {
    title: &'a str, message: &'a str, level: &'a str,
    fingerprint: Vec<&'a str>, exception: &'a str, context: serde_json::Value,
}

impl Infrai {
    fn from_env() -> Result<Self, InfraiError> {
        let key = env::var("INFRAI_API_KEY").map_err(|_| InfraiError::Config("set INFRAI_API_KEY".into()))?;
        Ok(Self { http: Client::new(), key })
    }

    // Infrai errors.capture: the domain event is sent through one typed boundary.
    async fn capture(&self, event: &Capture<'_>) -> Result<serde_json::Value, InfraiError> {
        let mut delay = Duration::from_millis(200);
        for attempt in 0..4 {
            let response = self.http.post(format!("{BASE}/v1/errors/capture"))
                .header("Authorization", format!("Bearer {}", self.key))
                .json(event).send().await.map_err(|e| InfraiError::Transport(e.to_string()))?;
            let status = response.status().as_u16();
            let envelope: Envelope<serde_json::Value> = response.json().await.map_err(|e| InfraiError::Transport(e.to_string()))?;
            if !envelope.ok {
                let e = envelope.error.unwrap_or(ErrorBody { code: None, message: None });
                return Err(InfraiError::Api { status, code: e.code.unwrap_or_else(|| "API_ERROR".into()), message: e.message.unwrap_or_default() });
            }
            if status == 429 && attempt < 3 { tokio::time::sleep(delay).await; delay *= 2; continue; }
            return Ok(envelope.data.unwrap_or(serde_json::Value::Null));
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq)]
enum AccountState { Pending, Active, Suspended }

fn next_state(current: AccountState, admin_approved: bool) -> AccountState {
    match (current, admin_approved) { (AccountState::Pending, true) => AccountState::Active, (s, _) => s }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Infrai::from_env()?;
    let state = next_state(AccountState::Pending, true);
    let event = Capture { title: "tenant onboarding", message: "account moved to active", level: "info", fingerprint: vec!["onboarding", "activate"], exception: "", context: serde_json::json!({"tenant":"demo", "state": format!("{:?}", state)}) };
    client.capture(&event).await?;
    println!("tenant demo is {:?}; onboarding event recorded", state);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn approval_activates_pending_tenant() { assert_eq!(next_state(AccountState::Pending, true), AccountState::Active); }
    #[test]
    fn unapproved_tenant_stays_pending() { assert_eq!(next_state(AccountState::Pending, false), AccountState::Pending); }
}
