use super::*;
use crate::models::{device::*, network::*};
use axum::http::Uri;
use libipam::response_error::{Builder, ResponseError};

use std::net::IpAddr;

pub async fn create(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Json(device): Json<models_data_entry::Device>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .instance(uri.to_string())
            .detail(format!("User doesn't belong to the {:?} role", Role::Admin))
            .build());
    }

    let state = state.lock().await;

    Ok(state.insert::<Device>(vec![device.into()]).await?)
}

pub async fn create_all_devices(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Path(network_id): Path<Uuid>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;
    let network = state
        .get::<Network>(Some(HashMap::from([("id", network_id.into())])))
        .await?
        .remove(0);

    match models_data_entry::create_all_devices(network.network, network_id) {
        Some(e) => Ok(state.insert::<Device>(e).await?),
        None => Err(ResponseError::builder()
            .status(StatusCode::NO_CONTENT)
            .build()),
    }
}

pub async fn get_all(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(network_id): Path<Uuid>,
) -> Result<QueryResult<Device>, ResponseError> {
    let state = state.lock().await;
    let condition = HashMap::from([("network_id", network_id.into())]);
    let devices = state.get::<Device>(Some(condition)).await.map_err(|x| {
        let tmp: Builder = ResponseError::from(x).into();
        tmp.instance(uri.to_string()).build()
    })?;

    Ok(devices.into())
}

pub async fn update(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Query(query_params::ParamDevice { ip, network_id }): Query<query_params::ParamDevice>,
    Json(mut device): Json<UpdateDevice>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }
    let state = state.lock().await;

    let current_data = state
        .get::<Device>(Some(HashMap::from([
            ("ip", ip.into()),
            ("network_id", network_id.into()),
        ])))
        .await?
        .remove(0);

    let mut netw_new = state
        .get::<Network>(Some(HashMap::from([(
            "id",
            match device.network_id {
                Some(e) => e.into(),
                None => network_id.into(),
            },
        )])))
        .await?
        .remove(0);

    let to_reduce = if let Some(true) = device
        .status
        .clone()
        .map(|_| current_data.status != Status::Unknown)
    {
        return Err(ResponseError::builder()
            .instance(uri.to_string())
            .status(StatusCode::BAD_REQUEST)
            .detail("You can change the status of devices with an unknown status".to_string())
            .build());
    } else {
        Some(1)
    };

    let device_to_delete = match (device.ip, device.network_id) {
        (Some(ip), Some(network_id))
            if ip == current_data.ip || network_id == current_data.network_id =>
        {
            Some((ip, network_id))
        }
        (Some(ip), None) if ip != current_data.ip => Some((ip, current_data.network_id)),
        (None, Some(network_id)) if network_id != current_data.network_id => {
            Some((current_data.ip, network_id))
        }
        _ => {
            device.ip = None;
            device.network_id = None;
            None
        }
    };

    if let Some((ip, network_id)) = device_to_delete {
        if let Ok(true) = state
            .get::<Device>(Some(HashMap::from([
                ("ip", ip.into()),
                ("network_id", network_id.into()),
            ])))
            .await
            .map(|mut x| x.remove(0))
            .map(|x| x.status != Status::Unknown)
        {
            return Err(ResponseError::builder()
                .status(StatusCode::BAD_REQUEST)
                .title(StatusCode::BAD_REQUEST.to_string())
                .instance(uri.to_string())
                .build());
        }

        let _ = state
            .delete::<Device>(Some(HashMap::from([
                ("ip", ip.into()),
                ("network_id", network_id.into()),
            ])))
            .await;
    }

    let resp = state
        .update::<Device, _>(
            device,
            Some(HashMap::from([
                ("ip", ip.into()),
                ("network_id", network_id.into()),
            ])),
        )
        .await?;
    if let Some(num) = to_reduce {
        netw_new.free.sub(num as u32).map_err(|e| {
            ResponseError::builder()
                .detail(
                    "The number from the free ip cannot be updated, but the device status can"
                        .to_string(),
                )
                .status(StatusCode::OK)
                .title(format!(
                    "Error to update free devices in the network {} - {:?}",
                    netw_new.network, e
                ))
                .instance(uri.to_string())
                .build()
        })?;

        netw_new.used.add(num as u32).map_err(|x| {
            ResponseError::builder()
                .detail(
                    "The number from the device used cannot be updated, but the device status can"
                        .to_string(),
                )
                .status(StatusCode::OK)
                .title(format!(
                    "Error to update ip used in the network {} - {:?}",
                    netw_new.network, x
                ))
                .instance(uri.to_string())
                .build()
        })?;

        let update_count = UpdateNetworkCount {
            used: Some(netw_new.used),
            free: Some(netw_new.free),
            available: None,
        };

        state
            .update::<Network, _>(
                update_count,
                Some(HashMap::from([("id", netw_new.id.into())])),
            )
            .await
            .map_err(|x| {
                let tmp: Builder = ResponseError::from(x).into();
                tmp.instance(uri.to_string()).build()
            })?;
    }
    Ok(resp)
}

pub async fn get_one(
    State(state): State<RepositoryType>,
    Query(query_params::ParamDevice { ip, network_id }): Query<query_params::ParamDevice>,
) -> Result<impl IntoResponse, ResponseError> {
    let state = state.lock().await;

    let device = state
        .get::<Device>(Some(HashMap::from([
            ("ip", ip.into()),
            ("network_id", network_id.into()),
        ])))
        .await?;

    Ok(Json(json!({
        "device": device.first()
    })))
}

pub async fn delete(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Query((ip, network_id)): Query<(IpAddr, Uuid)>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;

    Ok(state
        .delete::<Device>(Some(HashMap::from([
            ("ip", ip.into()),
            ("network_id", network_id.into()),
        ])))
        .await?)
}
