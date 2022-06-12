use std::cell::Cell;
use std::env;

use anyhow::{anyhow, bail, Result};
use reqwest::{blocking, header};
use serde_json::{json, Value as JsonValue};

type Token = String;

#[derive(Debug)]
pub enum Data {
    Heat {
        heat_point: f64,
        temperature: f64,
    },
    Cool {
        cool_point: f64,
        temperature: f64,
    },
    HeatCool {
        heat_point: f64,
        temperature: f64,
        cool_point: f64,
    },
    Off {
        temperature: f64,
    },
}

pub struct Client {
    token: Cell<Option<String>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            token: Cell::new(None),
        }
    }

    pub fn get_data(&self) -> Result<Data> {
        let project_id = env::var("NEST_PROJECT_ID")?;
        let nest_device_id = env::var("NEST_DEVICE_ID")?;
        let api_uri = format!(
            "https://smartdevicemanagement.googleapis.com/v1/enterprises/{project_id}/devices/{nest_device_id}");

        let client = blocking::Client::new();
        let traits = {
            let response: JsonValue = client
                .get(&api_uri)
                .header(header::AUTHORIZATION, self.token()?)
                .send()?
                .json()?;

            // Try refreshing token if request fails
            if response.get("error").is_some() {
                client
                    .get(&api_uri)
                    .header(header::AUTHORIZATION, self.refresh_token()?)
                    .send()
                    .and_then(blocking::Response::error_for_status)?
                    .json()?
            } else {
                response
            }
        }["traits"]
            .take();

        traits.try_into()
    }

    fn token(&self) -> Result<Token> {
        let token = match self.token.take() {
            Some(token) => {
                self.token.set(Some(token.clone()));
                token
            }
            None => self.refresh_token()?,
        };
        Ok(token)
    }

    fn refresh_token(&self) -> Result<Token> {
        const URL: &str = "https://www.googleapis.com/oauth2/v4/token";

        let client = blocking::Client::new();
        let request = json!({
            "client_id": env::var("NEST_CLIENT_ID")?,
            "client_secret": env::var("NEST_CLIENT_SECRET")?,
            "refresh_token": env::var("NEST_REFRESH_TOKEN")?,
            "grant_type": "refresh_token"
        });
        println!("{request}");
        let response: JsonValue = client
            .post(URL)
            .json(&request)
            .send()
            .and_then(blocking::Response::error_for_status)?
            .json()?;

        let token_type = response["token_type"]
            .as_str()
            .ok_or(anyhow!("No token type"))?;
        let access_token = response["access_token"]
            .as_str()
            .ok_or(anyhow!("No access token"))?;
        let token = format!("{token_type} {access_token}");
        self.token.set(Some(token.clone()));

        Ok(token)
    }
}

impl TryFrom<JsonValue> for Data {
    type Error = anyhow::Error;

    fn try_from(traits: JsonValue) -> Result<Self, Self::Error> {
        let range_trait = {
            let mode = traits["sdm.devices.traits.ThermostatEco"]["mode"].as_str();
            match mode {
                Some("OFF") => "sdm.devices.traits.ThermostatTemperatureSetpoint",
                Some("MANUAL_ECO") => "sdm.devices.traits.ThermostatEco",
                _ => bail!("Failed to retrieve Eco status"),
            }
        };

        let heat_point = traits[range_trait]["heatCelsius"].as_f64();
        let temperature =
            traits["sdm.devices.traits.Temperature"]["ambientTemperatureCelsius"].as_f64();
        let cool_point = traits[range_trait]["coolCelsius"].as_f64();

        match (heat_point, temperature, cool_point) {
            (Some(heat_point), Some(temperature), None) => Ok(Data::Heat {
                heat_point,
                temperature,
            }),
            (None, Some(temperature), Some(cool_point)) => Ok(Data::Cool {
                temperature,
                cool_point,
            }),
            (Some(heat_point), Some(temperature), Some(cool_point)) => Ok(Data::HeatCool {
                heat_point,
                temperature,
                cool_point,
            }),
            (None, Some(temperature), None) => Ok(Data::Off { temperature }),
            _ => Err(anyhow!("Failed to read temperature from Nest")),
        }
    }
}
