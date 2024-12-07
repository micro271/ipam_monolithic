use super::*;
use crate::models::{device::Device, network::*};
use axum::http::Uri;
use ipnet::IpNet;
use libipam::{ipam_services::subnetting, type_net::host_count::HostCount};
use query_params::ParamSubnetting;
use response_error::Builder;

pub async fn create(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Json(netw): Json<models_data_entry::Network>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .detail(format!("User {} not authorizedh", claim.sub))
            .title("Unauthorized".into())
            .instance(uri.to_string())
            .build());
    }

    let state = state.lock().await;
    tracing::info!("New network {:?}", netw);
    Ok(state.insert::<Network>(vec![netw.into()]).await?)
}

pub async fn get_one(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;
    let condition = HashMap::from([("id", id.into())]);

    Ok(state
        .get::<Network>(Some(condition))
        .await
        .map_err(|x| {
            let bl: Builder = ResponseError::from(x).into();
            bl.instance(uri.to_string()).build()
        })?
        .into())
}

pub async fn update(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Path(id): Path<Uuid>,
    Json(network): Json<UpdateNetwork>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .detail(format!("User {} not authorizedh", claim.sub))
            .title("Unauthorized".into())
            .instance(uri.to_string())
            .build());
    }

    let state = state.lock().await;

    if network.network.is_some() {
        return Err(ResponseError::builder()
            .title("Update not allowed".to_string())
            .instance(uri.to_string())
            .detail(
                "The function update_network still doesn't support the update for network"
                    .to_string(),
            )
            .status(StatusCode::NOT_IMPLEMENTED)
            .build());
    }

    Ok(state
        .update::<Network, _>(network, Some(HashMap::from([("id", id.into())])))
        .await?)
}

pub async fn get_all(
    State(state): State<RepositoryType>,
    uri: Uri,
) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;

    state
        .get::<Network>(Some(HashMap::from([("father", None::<Uuid>.into())])))
        .await
        .map(QueryResult::from)
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}

pub async fn delete(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Result<QueryResult<Network>, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;
    let mut to_delete = Vec::new();
    to_delete.push(id);
    let mut pos = 0;
    while let Ok(children) = state
        .get::<Network>(Some(HashMap::from([("father", to_delete[pos].into())])))
        .await
    {
        for i in children {
            to_delete.push(i.id);
        }
        pos += 1;
    }

    state
        .delete::<Network>(Some(HashMap::from([("id", id.into())])))
        .await
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}

pub async fn create_network_child(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Query(ParamSubnetting { prefix, father_id }): Query<ParamSubnetting>,
) -> Result<QueryResult<Network>, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .detail(format!("User {} not authorizedh", claim.sub))
            .title("Unauthorized".into())
            .build());
    }
    let state = state.lock().await;
    let mut network = state
        .get::<Network>(Some(HashMap::from([("id", father_id.into())])))
        .await
        .map_err(|x| {
            Into::<Builder>::into(ResponseError::from(x))
                .instance(uri.to_string())
                .build()
        })?
        .remove(0);

    if network.network.addr().is_ipv4() {
        match subnetting(network.network, prefix) {
            Ok(e) => {
                let new_networks = e
                    .into_iter()
                    .map(|x| Network {
                        id: uuid::Uuid::new_v4(),
                        father: Some(network.id),
                        vlan: None,
                        network: x,
                        description: None,
                        available: HostCount::new((&x).into()),
                        used: 0.into(),
                        free: HostCount::new((&x).into()),
                    })
                    .collect::<Vec<Network>>();

                let len = (new_networks.len() * 2) - /* We add two because the main network doesn't lose any addresses */ 2;

                match state.insert::<Network>(new_networks).await {
                    Ok(e) => {
                        let mut id_to_update = Some(network.id);
                        while let Some(network_to_update) = id_to_update {
                            let resp_sub = network.available.sub(len as u32);
                            if resp_sub.is_err() {
                                break;
                            }

                            let upd = UpdateNetworkCount {
                                used: None,
                                free: Some(network.available.clone()),
                                available: Some(network.available.clone()),
                            };

                            let _ = state
                                .update::<Network, _>(
                                    upd,
                                    Some(HashMap::from([("id", network_to_update.into())])),
                                )
                                .await;
                            if network.father.is_some() {
                                network = state
                                    .get::<Network>(Some(HashMap::from([(
                                        "id",
                                        network.father.into(),
                                    )])))
                                    .await?
                                    .remove(0);
                                id_to_update = Some(network.id);
                            } else {
                                id_to_update = None;
                            }
                        }
                        Ok(e)
                    }
                    Err(e) => Err(Into::<Builder>::into(ResponseError::from(e))
                        .instance(uri.to_string())
                        .build()),
                }
            }
            Err(e) => Err(ResponseError::builder()
                .title("We've had an error creating te subnet".to_string())
                .detail(e.to_string())
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .build()),
        }
    } else {
        let prefix_len = network.network.prefix_len();
        let max_prefix_len = network.network.max_prefix_len();

        let ip = format!("{}/{}", network.network.addr(), prefix)
            .parse::<IpNet>()
            .unwrap();
        if !network.network.contains(&ip) {
            return Err(ResponseError::builder()
                .title("Invalid network".to_string())
                .detail(format!(
                    "The network {} don't belong to the network {}",
                    network.network, ip
                ))
                .status(StatusCode::BAD_REQUEST)
                .instance(uri.to_string())
                .build());
        }

        let new_network = Network {
            id: uuid::Uuid::new_v4(),
            father: Some(network.id),
            vlan: None,
            network: ip,
            description: None,
            available: HostCount::new((&ip).into()),
            used: 0.into(),
            free: HostCount::new((&ip).into()),
        };

        let new = state
            .insert::<Network>(vec![new_network])
            .await
            .map_err(|x| {
                Into::<Builder>::into(ResponseError::from(x))
                    .instance(uri.to_string())
                    .build()
            })?;

        let avl_to_subtract = 2u128.pow((prefix - prefix_len) as u32);
        let new_avl = 2u128.pow((max_prefix_len - prefix_len) as u32) - avl_to_subtract + 2;
        if new_avl < HostCount::MAX as u128 {
            let new_avl: HostCount = (new_avl as u32).into();
            let updater = UpdateNetworkCount {
                available: Some(new_avl.clone()),
                used: None,
                free: Some(new_avl),
            };
            let _ = state
                .update::<Network, _>(updater, Some(HashMap::from([("id", network.id.into())])))
                .await?;
        }
        Ok(new)
    }
}

pub async fn get_all_with_father(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;

    state
        .get::<Network>(Some(HashMap::from([("father", id.into())])))
        .await
        .map(QueryResult::from)
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}

pub async fn clean(
    State(state): State<RepositoryType>,
    uri: Uri,
    Extension(claim): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
) -> Result<QueryResult<Network>, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .detail(format!("User {} not authorizedh", claim.sub))
            .title("Unauthorized".into())
            .instance(uri.to_string())
            .build());
    }

    let state = state.lock().await;
    let mut count = 0;

    if let QueryResult::Delete(e) = state
        .delete::<Network>(Some(HashMap::from([("father", id.into())])))
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?
    {
        count += e;
    }

    if let QueryResult::Delete(e) = state
        .delete::<Device>(Some(HashMap::from([("network_id", id.into())])))
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?
    {
        count += e;
    }

    if count > 0 {
        let network = state
            .get::<Network>(Some(HashMap::from([("id", id.into())])))
            .await
            .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?
            .remove(0);

        let avl = HostCount::new((&network.network).into());
        let updater = UpdateNetworkCount {
            available: Some(avl.clone()),
            free: Some(avl),
            used: Some(0.into()),
        };

        state
            .update::<Network, _>(updater, Some(HashMap::from([("id", network.id.into())])))
            .await?;
    }

    Ok(QueryResult::Delete(count))
}
