#[macro_use]
extern crate serde;
extern crate serde_json;

use nakama_rust_gen::nakama::api::{WriteStorageObject, WriteStorageObjectsRequest};

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
        name: "Storm Sword".to_string(),
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
            permission_write: Some(0), // "Owner Write" (1), or "No Write" (0)
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
    assert!(true)
}
