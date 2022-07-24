use serde_json;
use tauri::{command, State};

use crate::{api::remote_conn::send, Connection, Token};
use shared::{
    remote_conn_packet::RemotePacket,
    user::{UserRaw, UserSecure},
};

#[command]
pub async fn add_user(
    user: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    // verify user has valid properties
    let user: UserRaw = match serde_json::from_str(&user) {
        Ok(u) => u,
        Err(e) => return Err(s!(e)),
    };

    let user = serde_json::to_string(&user).unwrap();

    let data = match send(&conn, RemotePacket::new_with_auth(160, vec![user], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}

#[command]
pub async fn get_user_list(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(162, vec![s!("")], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s![e]),
    };

    Ok(data.payload[0].clone())
}

#[command]
pub async fn remove_user(
    name: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(163, vec![name], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}

#[command]
pub async fn update_user_group(
    user: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let user: UserSecure = match serde_json::from_str(&user) {
        Ok(u) => u,
        Err(e) => return Err(s!(e)),
    };

    let user = serde_json::to_string(&user).unwrap();

    let data = match send(&conn, RemotePacket::new_with_auth(164, vec![user], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}

#[command]
pub async fn update_user_password(
    user: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let user: UserRaw = match serde_json::from_str(&user) {
        Ok(u) => u,
        Err(e) => return Err(s!(e)),
    };

    let user = serde_json::to_string(&user).unwrap();

    let data = match send(&conn, RemotePacket::new_with_auth(165, vec![user], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}
