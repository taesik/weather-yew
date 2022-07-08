use js_sys::JsString;
use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use web_sys::console;
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub periods: Vec<Period>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub is_daytime: bool,
    pub temperature: f32,
    pub temperature_unit: String,
    pub wind_speed: String,
    pub wind_direction: String,
    pub icon: String,
    pub short_forecast: String,
    pub detailed_forecast: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Forecast {
    pub properties: Properties,
}

#[derive(PartialEq, Properties)]
struct PeriodComponentProps {
    pub period: Period,
}

#[function_component(PeriodComponent)]
fn period_component(props: &PeriodComponentProps) -> Html {
    let PeriodComponentProps { period } = props;
    html! {
        <div class="period">
            <div class="name">{period.name.to_owned()}</div>
            <div class="temp">{period.temperature.to_owned()}</div>
            <div class="forecast">{period.short_forecast.to_owned()}</div>
            // { period.start_time.to_owned() }
            <img src={props.period.icon.to_owned()} alt={""}/>
        </div>
    }
}

#[function_component(App)]
fn app_component() -> Html {
    let forecast:UseStateHandle<Option<Forecast>> = use_state(|| None);
    let forecast_clone = forecast.clone();
    let error = use_state(|| None);
    let error_clone = error.clone();
    let retry = {
        let forecast = forecast.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let forecast = forecast.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let forecast_endpoint = format!(
                    "https://api.weather.gov/gridpoints/{office}/{x},{y}/forecast",
                    office = "DTX",
                    x = 65,
                    y = 33
                );
                let fetched_forecast: Result<Response, Error> =
                    Request::get(&forecast_endpoint).send().await;

                match fetched_forecast {
                    Ok(response) => {
                        let json: Result<Forecast, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                forecast.set(Some(f));
                            }
                            Err(err) => {
                                error.set(Some(err.to_string()));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }

                // console::log_1(&JsString::from(
                //     serde_json::to_string(&fetched_forecast).unwrap())
                // );
                // forecast.set(Some(fetched_forecast));
            });
        })
    };
    match forecast_clone.as_ref() {
        Some(f) => {
            f.properties
                .periods
                .iter()
                .map(|period| {
                    return html! {
                   <PeriodComponent period={period.clone()}/>
                    }
                })
                .collect()
        },
        None => match error_clone.as_ref() {
            Some(e) => {
                return html! {
                    <>
                        <div>"error" {e}</div>
                        <button onclick={retry}>{"retry"}</button>
                    </>

                }
            }
            None => {
                return html! (
                    <>
                        {"No data yet"}
                        <button onclick={retry}>{"Load Api"}</button>
                    </>
                )
            }
        },
    }
}

fn main() {
    yew::start_app::<App>();
}
