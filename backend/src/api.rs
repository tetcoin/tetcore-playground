//! HTTP endpoints exposed in /api context

use crate::Context;
use rocket::{get, post, State};
use rocket_contrib::{json, json::JsonValue};
use serde::Serialize;

// TODO add image templates endpoint

fn result_to_jsonrpc<T: Serialize>(res: Result<T, String>) -> JsonValue {
    match res {
        Ok(val) => json!({ "result": val }),
        Err(err) => json!({ "error": err }),
    }
}

#[get("/<user_uuid>")]
pub fn list(state: State<'_, Context>, user_uuid: String) -> JsonValue {
    let manager = state.manager.clone();
    result_to_jsonrpc(manager.list(&user_uuid))
}

#[get("/<user_uuid>/<instance_uuid>")]
pub fn get(state: State<'_, Context>, user_uuid: String, instance_uuid: String) -> JsonValue {
    let manager = state.manager.clone();
    result_to_jsonrpc(manager.get(&user_uuid, &instance_uuid))
}

/// Deploy `template` Docker container for `user_uuid`.
#[post("/<user_uuid>?<template>")]
pub fn deploy(state: State<'_, Context>, user_uuid: String, template: String) -> JsonValue {
    let manager = state.manager.clone();
    result_to_jsonrpc(manager.deploy(&user_uuid, &template))
}
