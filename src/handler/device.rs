use super::*;
use crate::models::{device::*, network::*};
use axum::http::Uri;
use libipam::ipam_services::{self, Ping};

use super::{response_error::Builder, ResponseError};

use std::{collections::HashSet, net::IpAddr};

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

    if let Ok(devs) = state
        .get::<Device>(Some(HashMap::from([("network_id", network_id.into())])))
        .await
    {
        let ips = devs.into_iter().map(|x| x.ip).collect::<HashSet<IpAddr>>();

        if ips.len() != *network.available as usize {
            let to_insert: Vec<Device> = network
                .network
                .hosts()
                .filter(|ip| !ips.contains(ip))
                .map(|ip| Device {
                    ip,
                    description: None,
                    location: None,
                    status: Status::default(),
                    network_id,
                    credential: None,
                })
                .collect();
            Ok(state.insert(to_insert).await?)
        } else {
            Err(ResponseError::builder()
                .title("The devices alreade exist".to_string())
                .detail("We haven't found any missing devices".to_string())
                .status(StatusCode::BAD_REQUEST)
                .build())
        }
    } else {
        match models_data_entry::create_all_devices(network.network, network_id) {
            Some(e) => Ok(state.insert::<Device>(e).await?),
            None => Err(ResponseError::builder()
                .status(StatusCode::NO_CONTENT)
                .build()),
        }
    }
}

pub async fn get_all(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(network_id): Path<Uuid>,
) -> Result<QueryResult<Device>, ResponseError> {
    let state = state.lock().await;
    let condition = HashMap::from([("network_id", network_id.into())]);
    let mut devices = state.get::<Device>(Some(condition)).await.map_err(|x| {
        let tmp: Builder = ResponseError::from(x).into();
        tmp.instance(uri.to_string()).build()
    })?;
    devices.sort_by_key(|x| x.ip);

    Ok(devices.into())
}

pub async fn update(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Query(query_params::ParamDevice { ip, network_id }): Query<query_params::ParamDevice>,
    Json(device): Json<UpdateDevice>,
) -> Result<QueryResult<Device>, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .instance(uri.to_string())
            .detail(format!("The user {} isn't Admin", claim.username))
            .title("Unauthorized".to_string())
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }
    let state = state.lock().await;

    let network = state
        .get::<Network>(Some(HashMap::from([(
            "id",
            match device.network_id {
                Some(e) => e,
                None => network_id,
            }
            .into(),
        )])))
        .await?
        .remove(0);

    if device.ip.is_some() || device.network_id.is_some() {
        if device.ip.as_ref().map(|x| x != &ip).unwrap_or(false)
            || device.network_id.map(|x| x != network_id).unwrap_or(false)
        {
            let ip_to_delete = match device.ip {
                Some(e) => e,
                _ => ip,
            };
            if network.network.contains(&ip_to_delete) {
                state
                    .delete::<Device>(Some(HashMap::from([
                        ("ip", ip_to_delete.into()),
                        ("network_id", network.id.into()),
                    ])))
                    .await?;
            } else {
                return Err(ResponseError::builder()
                    .detail(format!(
                        "Ip {:?} or network {:?} is not compatible",
                        device.ip, network.network
                    ))
                    .title("Conflict".to_string())
                    .status(StatusCode::BAD_REQUEST)
                    .build());
            }
        } else {
            return Err(ResponseError::builder()
                .detail(
                    "The data entry (ip or network) is the same as the data in the database"
                        .to_string(),
                )
                .title("Nothing happend".to_string())
                .status(StatusCode::BAD_REQUEST)
                .build());
        }
    }

    Ok(state
        .update::<Device, _>(
            device,
            Some(HashMap::from([
                ("ip", ip.into()),
                ("network_id", network_id.into()),
            ])),
        )
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?)
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

pub async fn ping(
    State(state): State<RepositoryType>,
    uri: Uri,
    Extension(_claims): Extension<Claims>,
    Query(query_params::ParamDevice { ip, network_id }): Query<query_params::ParamDevice>,
) -> Result<Ping, ResponseError> {
    let state = state.lock().await;

    let device = state
        .get::<Device>(Some(HashMap::from([
            ("ip", ip.into()),
            ("network_id", network_id.into()),
        ])))
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?
        .remove(0);

    if ipam_services::Ping::Pong == ipam_services::ping(ip, 1000).await {
        if device.status != Status::Online {
            state
                .update::<Device, _>(
                    Status::Online,
                    Some(HashMap::from([
                        ("ip", ip.into()),
                        ("network_id", network_id.into()),
                    ])),
                )
                .await
                .map_err(|x| {
                    Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string())
                })?;
        }

        if device.status == Status::Unknown {
            let mut network = state
                .get::<Network>(Some(HashMap::from([("id", network_id.into())])))
                .await
                .map_err(|x| {
                    Into::<Builder>::into(ResponseError::from(x))
                        .instance(uri.to_string())
                        .title("We can't changed the network count".to_string())
                })?
                .remove(0);

            if network.used.add(1_u8).is_err() || network.free.sub(1_u8).is_err() {
                return Err(ResponseError::builder()
                    .detail(format!(
                        "We've been able to modified the free and used counter in network {}",
                        network.network
                    ))
                    .title("Fail to modifie host counter".to_string())
                    .status(StatusCode::NOT_MODIFIED)
                    .build());
            }

            let updater = UpdateNetworkCount {
                used: Some(network.used.clone()),
                free: Some(network.free.clone()),
                available: None,
            };
            if state
                .update::<Network, _>(updater, Some(HashMap::from([("id", network_id.into())])))
                .await
                .is_err()
            {
                return Err(ResponseError::builder()
                    .instance(uri.to_string())
                    .status(StatusCode::NOT_MODIFIED)
                    .detail("We have been able to modify the host counter in the network, but the device was updated seamlessly".to_string())
                    .title("We can't changed the network count".to_string())
                    .build());
            }
        }

        Ok(Ping::Pong)
    } else {
        if device.status == Status::Online {
            state
                .update::<Device, _>(
                    Status::Offline,
                    Some(HashMap::from([
                        ("ip", ip.into()),
                        ("network_id", network_id.into()),
                    ])),
                )
                .await
                .map_err(|x| {
                    Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string())
                })?;
        }
        Ok(Ping::Fail)
    }
}

pub async fn reserve(
    State(state): State<RepositoryType>,
    uri: Uri,
    Query(query_params::ParamDevice { ip, network_id }): Query<query_params::ParamDevice>,
) -> Result<QueryResult<Device>, ResponseError> {
    let state = state.lock().await;

    let tmp = state
        .update(
            Status::Reserved,
            Some(HashMap::from([
                ("ip", ip.into()),
                ("network_id", network_id.into()),
            ])),
        )
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?;

    let mut network = state
        .get::<Network>(Some(HashMap::from([("id", network_id.into())])))
        .await
        .unwrap()
        .remove(0);
    let mut id_to_update = Some(network.id);

    while let Some(id) = id_to_update {
        let result_used = network.used.add(1_u32);
        let result_free = network.free.sub(1_u32);
        if result_free.is_err() && result_used.is_err() {
            return Err(ResponseError::builder()
                .title("Errir to update".into())
                .detail(format!(
                    "Error to update HostCounter in the network {}",
                    network.network
                ))
                .instance(uri.to_string())
                .build());
        }

        let updater = UpdateNetworkCount {
            used: Some(network.used.clone()),
            free: Some(network.free.clone()),
            available: None,
        };
        state
            .update::<Network, _>(updater, Some(HashMap::from([("id", id.into())])))
            .await
            .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?;

        if let Some(father) = network.father {
            network = state
                .get::<Network>(Some(HashMap::from([("id", father.into())])))
                .await
                .unwrap()
                .remove(0);
            id_to_update = Some(network.id);
        } else {
            id_to_update = None;
        }
    }

    Ok(tmp)
}
