// Copyright ⓒ 2024-2025 Peter Morgan <peter.james.morgan@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common::init_tracing;
use std::collections::BTreeMap;
use tansu_model::MessageKind;
use tansu_sans_io::MESSAGE_META;

pub mod common;

#[test]
fn check_message_meta() {
    let _guard = init_tracing().unwrap();

    assert!(BTreeMap::from(MESSAGE_META).contains_key("CreateTopicsRequest"));

    let meta = BTreeMap::from(MESSAGE_META);

    let message = meta.get("CreateTopicsRequest").unwrap();
    assert_eq!(19, message.api_key);
    assert_eq!(MessageKind::Request, message.message_kind);

    let structures = message.structures();
    let keys: Vec<&&str> = structures.keys().collect();
    assert_eq!(
        vec![
            &"CreatableReplicaAssignment",
            &"CreatableTopic",
            &"CreatableTopicConfig"
        ],
        keys
    );

    assert!(
        message
            .field("topics")
            .map(|field| field.kind.is_sequence())
            .unwrap()
    );

    assert!(
        message
            .field("topics")
            .map(|field| field.is_structure())
            .unwrap()
    );

    assert!(
        !message
            .field("topics")
            .map(|field| field.kind.is_primitive())
            .unwrap()
    );

    assert_eq!(
        "CreatableTopic",
        message
            .field("topics")
            .and_then(|field| field.kind.kind_of_sequence())
            .map(|kind_meta| kind_meta.0)
            .unwrap()
    );

    assert!(
        !message
            .field("timeout_ms")
            .map(|field| field.kind.is_sequence())
            .unwrap()
    );

    assert!(
        !message
            .field("timeout_ms")
            .map(|field| field.is_structure())
            .unwrap()
    );

    assert!(
        !message
            .field("validate_only")
            .map(|field| field.kind.is_sequence())
            .unwrap()
    );

    assert!(
        !message
            .field("validate_only")
            .map(|field| field.is_structure())
            .unwrap()
    );
}
