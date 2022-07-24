use tauri::{command, State};

use crate::{api::remote_conn::send, Connection, Token};
use shared::{launch::LaunchParams, remote_conn_packet::RemotePacket};

#[command]
pub async fn launch(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(69, vec![s!("")], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}

#[command]
pub async fn set_destination(
    params: String,
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let params = match serde_json::from_str::<LaunchParams>(&params) {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    let params = serde_json::to_string(&params).unwrap();

    let data = match send(
        &conn,
        RemotePacket::new_with_auth(68, vec![s!(params)], token),
    )
    .await
    {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}

#[command]
pub async fn stop(
    conn_state: State<'_, Connection>,
    token: State<'_, Token>,
) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(99, vec![s!("")], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e)),
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]]);
    }

    Ok(data.payload[0].clone())
}
