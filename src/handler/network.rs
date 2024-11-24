use super::*;
use crate::models::network::*;
use axum::http::Uri;
use libipam::{ipam_services::subnetting, response_error::{Builder, ResponseError}, type_net::host_count::HostCount};
use query_params::ParamSubnetting;
pub async fn create(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Json(netw): Json<models_data_entry::Network>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
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
        .get::<Network>(Some(HashMap::from([("father",None::<Uuid>.into())])))
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

    state
        .delete::<Network>(Some(HashMap::from([("id", id.into())])))
        .await
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}

pub async fn create_network_child(State(state): State<RepositoryType>, uri: Uri, Query(ParamSubnetting {prefix, father_id}): Query<ParamSubnetting>) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;
    let network = state.get::<Network>(Some(HashMap::from([("id",father_id.into())]))).await.map_err(|x| {
        Into::<Builder>::into(ResponseError::from(x))
            .instance(uri.to_string())
            .build()

    })?.remove(0);
    if (network.network.prefix_len() - prefix) <= 0 {
        return Err(ResponseError::builder()
            .title("Prefix invalid".to_string())
            .detail(format!("The subnet {}/{} is bigger than {}, therefore we've created it", network.network.addr(), prefix, network.network))
            .status(StatusCode::BAD_REQUEST)
            .build()
        );
    }
    match subnetting(subnetting.ip, subnetting.prefix).await {
        Ok(e) =>  {
            let new_networks = e.into_iter().map(|x| {
                Network {
                    id: uuid::Uuid::new_v4(),
                    father: Some(network.id),
                    vlan: None,
                    network: x,
                    description: None,
                    available: HostCount::new((&x).into()),
                    used: 0.into(),
                    free: HostCount::new((&x).into()),
                }
            }).collect::<Vec<Network>>();

            Ok(state.insert::<Network>(new_networks).await.map_err(|x| {
                Into::<Builder>::into(ResponseError::from(x))
                    .instance(uri.to_string())
                    .build()
            })?)
        },
        Err(e) => {
            Err(ResponseError::builder()
                .title("We've had an error creating te subnet".to_string())
                .detail(e.to_string())
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .build())
        }
    }
}

pub async fn get_all_with_father(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(id): Path<Uuid>
) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;

    state
        .get::<Network>(Some(HashMap::from([("father",id.into())])))
        .await
        .map(QueryResult::from)
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}