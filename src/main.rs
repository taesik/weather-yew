mod model;

use reqwasm::http::Request;
use yew::prelude::*;
use web_sys::console;
use js_sys::JsString;

#[function_component(App)]
fn app_component() ->Html {
    wasm_bindgen_futures::spawn_local(async move {
        let forecast_endpoint = format!(
            "https://api.weather.gov/gridpoints/{office}/{x},{y}/forecast",
            office="DTX",
            x = 65,
            y = 33
        );
        let fetched_forecast = Request::get(&forecast_endpoint)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    });


    html!(
        {"Hi"}
    )
}

fn main() {
    yew::start_app::<App>();
}
