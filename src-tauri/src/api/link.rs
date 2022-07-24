use anyhow::Result;
use serde_json;
use tauri::{command, State};

use crate::{api::remote_conn::*, Connection, Token};
use shared::{device::Device, remote_conn_packet::RemotePacket};

#[command]
pub async fn lock_devices(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(63, vec![s![""]], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    // return result to vue frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn unlock_devices(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // lock tauri state and ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    // send new device to pod computer
    let data = match send(&conn, RemotePacket::new_with_auth(62, vec![s![""]], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn add_device(
    dev: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // lock tauri state and ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    // verify device has valid properties
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(d) => d,
        Err(e) => return Err(s![e]),
    };

    let dev = serde_json::to_string(&dev).unwrap();

    // send new device to pod computer
    let data = match send(&conn, RemotePacket::new_with_auth(33, vec![dev], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn get_device_list(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(32, vec![s![""]], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    // return result to vue frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn remove_device(
    dev: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    // verify device has valid properties
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(d) => d,
        Err(e) => return Err(s![e]),
    };

    let dev = serde_json::to_string(&dev).unwrap();

    // send device to remove from pod computer
    let data = match send(&conn, RemotePacket::new_with_auth(35, vec![dev], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn update_device(
    dev: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    // verify device has valid properties
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(d) => d,
        Err(e) => return Err(s![e]),
    };

    let dev = serde_json::to_string(&dev).unwrap();

    // send device to update on pod computer
    let data = match send(&conn, RemotePacket::new_with_auth(34, vec![dev], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn get_pod_state(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(64, vec![s![""]], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    Ok(data.payload[0].clone())
}
