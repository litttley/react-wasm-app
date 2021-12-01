mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::prelude::*;
use js_sys::{AsyncIterator, Error, Function, Object, Promise};
use serde::{Deserialize, Serialize};
use serde_json::value::Index;
use serde_json::{json, Value};
use std::convert::TryInto;
use surf::http::{mime, Method};
use surf::Config;
use surf::Url;

pub async fn async_translate(
    url:String,
    value: String,
    from_lang: String,
    to_lang: String,
    times: u32,
) -> Result<JsValue, JsValue> {
   // let uri = "http://localhost:3000/translate";

    let value = json!({ "value": value,"from_lang":from_lang,"to_lang":to_lang,"times":times });
    #[derive(Deserialize, Serialize)]
    pub struct ResultMsg {
        pub code: i32,
        pub msg: String,
        pub data: String,
    }
    let mut res = surf::post(url).body_json(&value).unwrap().await.unwrap();
    let result: ResultMsg = res.body_json().await.unwrap();
    let body_str = result.data;

    let result = match &body_str.chars().skip(6).position(|c: char| c == '\n') {
        Some(n1) => {
            println!("n1:{:#?}", n1);
            let n2 = n1 + 6;
            println!("n2:{:#?}", n2);

            match &body_str
                .chars()
                .skip(6)
                .take(*n1)
                .collect::<String>()
                .parse::<usize>()
            {
                Ok(f) => {
                    println!("f:{:#?}", f);
                    let ss = body_str.chars().skip(n2).take(*f).collect::<String>();
                    println!("f:{:#?}", ss);

                    let jsonObj: serde_json::Value = serde_json::from_str(&ss).unwrap();
                    let content = match jsonObj {
                        Value::Array(array) => match &array[0][2] {
                            Value::String(value) => {
                                let json_value: serde_json::Value =
                                    serde_json::from_str(value).unwrap();
                                match json_value {
                                    Value::Array(arr) => match &arr[1][0][0][5] {
                                        Value::Array(arr) => {
                                            let content = arr
                                                .iter()
                                                .map(|value| value[0].to_string())
                                                .collect::<String>();
                                            let content = content.replace("\"", "");
                                            content
                                        }

                                        _ => "".to_string(),
                                    },
                                    _ => "".to_string(),
                                }
                            }
                            _ => "".to_string(),
                        },

                        _ => "".to_string(),
                    };
                    content
                }
                Err(_) => "位置解析错误:2".to_string(),
            }
        }
        None => "位置获取错误".to_string(),
    };

    Ok(JsValue::from_str(result.as_str()))
}
#[wasm_bindgen]
pub fn translate(url:String, value: String, from_lang: String, to_lang: String, times: u32) -> Promise {
    future_to_promise(async_translate(url,value, from_lang, to_lang, times))
}
