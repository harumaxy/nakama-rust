#[macro_use]
extern crate serde;
extern crate serde_json;

use nakama_rust_gen::nakama::api::{
    DeleteStorageObjectId, DeleteStorageObjectsRequest, ReadStorageObjectId,
    ReadStorageObjectsRequest, WriteStorageObject, WriteStorageObjectsRequest,
};
use serde::__private::de;

mod test_utils;

#[derive(Serialize, Deserialize, Debug)]
struct Weapon {
    name: String,
    atk: i32,
}

#[tokio::test]
async fn write_object() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let sword = Weapon {
        name: "Storm Sword⚔️".to_string(),
        atk: 999,
    };
    let value = serde_json::to_string(&sword).unwrap();
    println!("{}", value);

    let req = WriteStorageObjectsRequest {
        objects: [WriteStorageObject {
            collection: "equipment".into(),
            key: "weapon".into(),
            value: value.into(),
            permission_read: Some(2), // "Public Read" (2), "Owner Read" (1), or "No Read" (0).
            permission_write: Some(1), // "Owner Write" (1), or "No Write" (0)
            ..Default::default()
        }]
        .into(),
    };
    let response = client.write_storage_objects(req).await;

    assert!(response
        .map(|storage_obj_ack| println!("Ok： {:?}", storage_obj_ack))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}

#[tokio::test]
async fn read_object() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let self_account = client.get_account(()).await.unwrap().into_inner();
    let req = ReadStorageObjectsRequest {
        object_ids: [ReadStorageObjectId {
            collection: "equipment".into(),
            key: "weapon".into(),
            user_id: self_account.user.unwrap().id,
        }]
        .into(),
    };
    let response = client.read_storage_objects(req).await;
    assert!(response
        .map(|storage_objs| println!("Ok： {:?}", storage_objs))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}

#[tokio::test]
async fn delete_object() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let self_account = client.get_account(()).await.unwrap().into_inner();
    let req = ReadStorageObjectsRequest {
        object_ids: [ReadStorageObjectId {
            collection: "equipment".into(),
            key: "weapon".into(),
            user_id: self_account.user.unwrap().id,
        }]
        .into(),
    };
    let read_response = client.read_storage_objects(req).await.unwrap().into_inner();
    let delete_ids = read_response
        .objects
        .iter()
        .map(|r| DeleteStorageObjectId {
            collection: r.collection.clone(),
            key: r.key.clone(),
            version: r.version.clone(),
        })
        .collect();
    let req = DeleteStorageObjectsRequest {
        object_ids: delete_ids,
    };
    let response = client.delete_storage_objects(req).await;
    assert!(response
        .map(|none| println!("Ok： {:?}", none))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}
