use crate::signal::{LeafType, Value, ValueType};
use crate::vehicle_shadow::VehicleShadow;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

// 生成されたprotoファイルをインポート
pub mod vehicle_shadow {
    tonic::include_proto!("vehicle_shadow");
}

use vehicle_shadow::signal_service_server::{SignalService, SignalServiceServer};
use vehicle_shadow::{
    GetRequest, GetResponse, SetRequest, SetResponse, SetResult, SubscribeRequest,
    SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse,
};

// 変換関数: protoのValue -> RustのValue
fn convert_proto_value_to_rust(proto_value: &vehicle_shadow::Value) -> crate::signal::Value {
    match &proto_value.value {
        Some(vehicle_shadow::value::Value::BoolValue(v)) => crate::signal::Value::Bool(*v),
        Some(vehicle_shadow::value::Value::StringValue(v)) => crate::signal::Value::String(v.clone()),
        Some(vehicle_shadow::value::Value::Int8Value(v)) => crate::signal::Value::Int8(*v as i8),
        Some(vehicle_shadow::value::Value::Int16Value(v)) => crate::signal::Value::Int16(*v as i16),
        Some(vehicle_shadow::value::Value::Int32Value(v)) => crate::signal::Value::Int32(*v),
        Some(vehicle_shadow::value::Value::Int64Value(v)) => crate::signal::Value::Int64(*v),
        Some(vehicle_shadow::value::Value::Uint8Value(v)) => crate::signal::Value::Uint8(*v as u8),
        Some(vehicle_shadow::value::Value::Uint16Value(v)) => crate::signal::Value::Uint16(*v as u16),
        Some(vehicle_shadow::value::Value::Uint32Value(v)) => crate::signal::Value::Uint32(*v),
        Some(vehicle_shadow::value::Value::Uint64Value(v)) => crate::signal::Value::Uint64(*v),
        Some(vehicle_shadow::value::Value::FloatValue(v)) => crate::signal::Value::Float(*v),
        Some(vehicle_shadow::value::Value::DoubleValue(v)) => crate::signal::Value::Double(*v),
        Some(vehicle_shadow::value::Value::BoolArrayValue(v)) => crate::signal::Value::BoolArray(v.values.clone()),
        Some(vehicle_shadow::value::Value::StringArrayValue(v)) => crate::signal::Value::StringArray(v.values.clone()),
        Some(vehicle_shadow::value::Value::Int8ArrayValue(v)) => crate::signal::Value::Int8Array(v.values.iter().map(|&x| x as i8).collect()),
        Some(vehicle_shadow::value::Value::Int16ArrayValue(v)) => crate::signal::Value::Int16Array(v.values.iter().map(|&x| x as i16).collect()),
        Some(vehicle_shadow::value::Value::Int32ArrayValue(v)) => crate::signal::Value::Int32Array(v.values.clone()),
        Some(vehicle_shadow::value::Value::Int64ArrayValue(v)) => crate::signal::Value::Int64Array(v.values.clone()),
        Some(vehicle_shadow::value::Value::Uint8ArrayValue(v)) => crate::signal::Value::Uint8Array(v.values.iter().map(|&x| x as u8).collect()),
        Some(vehicle_shadow::value::Value::Uint16ArrayValue(v)) => crate::signal::Value::Uint16Array(v.values.iter().map(|&x| x as u16).collect()),
        Some(vehicle_shadow::value::Value::Uint32ArrayValue(v)) => crate::signal::Value::Uint32Array(v.values.clone()),
        Some(vehicle_shadow::value::Value::Uint64ArrayValue(v)) => crate::signal::Value::Uint64Array(v.values.clone()),
        Some(vehicle_shadow::value::Value::FloatArrayValue(v)) => crate::signal::Value::FloatArray(v.values.clone()),
        Some(vehicle_shadow::value::Value::DoubleArrayValue(v)) => crate::signal::Value::DoubleArray(v.values.clone()),
        None => crate::signal::Value::NAN,
    }
}

// 変換関数: RustのValue -> protoのValue
fn convert_value_to_proto(value: &Value) -> vehicle_shadow::Value {
    match value {
        Value::NAN => vehicle_shadow::Value { value: None },
        Value::Bool(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::BoolValue(*v)),
        },
        Value::String(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::StringValue(v.clone())),
        },
        Value::Int8(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int8Value(*v as i32)),
        },
        Value::Int16(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int16Value(*v as i32)),
        },
        Value::Int32(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int32Value(*v)),
        },
        Value::Int64(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int64Value(*v)),
        },
        Value::Uint8(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint8Value(*v as u32)),
        },
        Value::Uint16(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint16Value(*v as u32)),
        },
        Value::Uint32(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint32Value(*v)),
        },
        Value::Uint64(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint64Value(*v)),
        },
        Value::Float(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::FloatValue(*v)),
        },
        Value::Double(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::DoubleValue(*v)),
        },
        Value::BoolArray(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::BoolArrayValue(
                vehicle_shadow::BoolArray { values: v.clone() },
            )),
        },
        Value::StringArray(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::StringArrayValue(
                vehicle_shadow::StringArray { values: v.clone() },
            )),
        },
        Value::Int8Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int8ArrayValue(
                vehicle_shadow::Int8Array {
                    values: v.iter().map(|&x| x as i32).collect(),
                },
            )),
        },
        Value::Int16Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int16ArrayValue(
                vehicle_shadow::Int16Array {
                    values: v.iter().map(|&x| x as i32).collect(),
                },
            )),
        },
        Value::Int32Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int32ArrayValue(
                vehicle_shadow::Int32Array { values: v.clone() },
            )),
        },
        Value::Int64Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Int64ArrayValue(
                vehicle_shadow::Int64Array { values: v.clone() },
            )),
        },
        Value::Uint8Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint8ArrayValue(
                vehicle_shadow::Uint8Array {
                    values: v.iter().map(|&x| x as u32).collect(),
                },
            )),
        },
        Value::Uint16Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint16ArrayValue(
                vehicle_shadow::Uint16Array {
                    values: v.iter().map(|&x| x as u32).collect(),
                },
            )),
        },
        Value::Uint32Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint32ArrayValue(
                vehicle_shadow::Uint32Array { values: v.clone() },
            )),
        },
        Value::Uint64Array(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::Uint64ArrayValue(
                vehicle_shadow::Uint64Array { values: v.clone() },
            )),
        },
        Value::FloatArray(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::FloatArrayValue(
                vehicle_shadow::FloatArray { values: v.clone() },
            )),
        },
        Value::DoubleArray(v) => vehicle_shadow::Value {
            value: Some(vehicle_shadow::value::Value::DoubleArrayValue(
                vehicle_shadow::DoubleArray { values: v.clone() },
            )),
        },
    }
}

// 変換関数: RustのValueType -> protoのValueType
fn convert_value_type_to_proto(value_type: &ValueType) -> vehicle_shadow::ValueType {
    match value_type {
        ValueType::TypeNAN => vehicle_shadow::ValueType::TypeNan,
        ValueType::TypeBool => vehicle_shadow::ValueType::TypeBool,
        ValueType::TypeString => vehicle_shadow::ValueType::TypeString,
        ValueType::TypeInt8 => vehicle_shadow::ValueType::TypeInt8,
        ValueType::TypeInt16 => vehicle_shadow::ValueType::TypeInt16,
        ValueType::TypeInt32 => vehicle_shadow::ValueType::TypeInt32,
        ValueType::TypeInt64 => vehicle_shadow::ValueType::TypeInt64,
        ValueType::TypeUint8 => vehicle_shadow::ValueType::TypeUint8,
        ValueType::TypeUint16 => vehicle_shadow::ValueType::TypeUint16,
        ValueType::TypeUint32 => vehicle_shadow::ValueType::TypeUint32,
        ValueType::TypeUint64 => vehicle_shadow::ValueType::TypeUint64,
        ValueType::TypeFloat => vehicle_shadow::ValueType::TypeFloat,
        ValueType::TypeDouble => vehicle_shadow::ValueType::TypeDouble,
        ValueType::TypeBoolArray => vehicle_shadow::ValueType::TypeBoolArray,
        ValueType::TypeStringArray => vehicle_shadow::ValueType::TypeStringArray,
        ValueType::TypeInt8Array => vehicle_shadow::ValueType::TypeInt8Array,
        ValueType::TypeInt16Array => vehicle_shadow::ValueType::TypeInt16Array,
        ValueType::TypeInt32Array => vehicle_shadow::ValueType::TypeInt32Array,
        ValueType::TypeInt64Array => vehicle_shadow::ValueType::TypeInt64Array,
        ValueType::TypeUint8Array => vehicle_shadow::ValueType::TypeUint8Array,
        ValueType::TypeUint16Array => vehicle_shadow::ValueType::TypeUint16Array,
        ValueType::TypeUint32Array => vehicle_shadow::ValueType::TypeUint32Array,
        ValueType::TypeUint64Array => vehicle_shadow::ValueType::TypeUint64Array,
        ValueType::TypeFloatArray => vehicle_shadow::ValueType::TypeFloatArray,
        ValueType::TypeDoubleArray => vehicle_shadow::ValueType::TypeDoubleArray,
    }
}

// 変換関数: RustのLeafType -> protoのLeafType
fn convert_leaf_type_to_proto(leaf_type: &LeafType) -> vehicle_shadow::LeafType {
    match leaf_type {
        LeafType::Branch => vehicle_shadow::LeafType::Branch,
        LeafType::Sensor => vehicle_shadow::LeafType::Sensor,
        LeafType::Attribute => vehicle_shadow::LeafType::Attribute,
        LeafType::Actuator => vehicle_shadow::LeafType::Actuator,
    }
}

// 変換関数: RustのSignal -> protoのSignal
fn convert_signal_to_proto(signal: &crate::signal::Signal) -> vehicle_shadow::Signal {
    vehicle_shadow::Signal {
        path: signal.path.clone(),
        state: Some(vehicle_shadow::State {
            value: Some(convert_value_to_proto(&signal.state.value)),
            capability: Some(signal.state.capability),
            availability: Some(signal.state.availability),
            reserved: Some(signal.state.reserved.clone()),
        }),
        config: Some(vehicle_shadow::Config {
            leaf_type: convert_leaf_type_to_proto(&signal.config.leaf_type) as i32,
            data_type: convert_value_type_to_proto(&signal.config.data_type) as i32,
            deprecation: signal.config.deprecation.clone(),
            unit: signal.config.unit.clone(),
            min: signal.config.min.as_ref().map(convert_value_to_proto),
            max: signal.config.max.as_ref().map(convert_value_to_proto),
            description: signal.config.description.clone(),
            comment: signal.config.comment.clone(),
            allowd: signal
                .config
                .allowd
                .as_ref()
                .map(|v| v.iter().map(convert_value_to_proto).collect())
                .unwrap_or_default(),
            default: signal.config.default.as_ref().map(convert_value_to_proto),
            end_point: signal.config.end_point.clone(),
        }),
    }
}

// 購読管理用の構造体
#[derive(Default)]
pub struct SubscriptionManager {
    subscriptions: HashMap<String, Vec<tokio::sync::mpsc::Sender<SubscribeResponse>>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    pub fn subscribe(
        &mut self,
        path: String,
    ) -> tokio::sync::mpsc::Receiver<SubscribeResponse> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        self.subscriptions
            .entry(path)
            .or_insert_with(Vec::new)
            .push(tx);
        rx
    }

    pub fn unsubscribe(&mut self, path: &str) {
        self.subscriptions.remove(path);
    }

    pub fn notify(&self, path: &str, response: SubscribeResponse) {
        if let Some(senders) = self.subscriptions.get(path) {
            for sender in senders {
                let _ = sender.try_send(response.clone());
            }
        }
    }
}

// SignalServiceの実装
pub struct SignalServiceImpl {
    vehicle_shadow: Arc<RwLock<VehicleShadow>>,
    subscription_manager: Arc<RwLock<SubscriptionManager>>,
}

impl SignalServiceImpl {
    pub fn new(vehicle_shadow: VehicleShadow) -> Self {
        Self {
            vehicle_shadow: Arc::new(RwLock::new(vehicle_shadow)),
            subscription_manager: Arc::new(RwLock::new(SubscriptionManager::new())),
        }
    }
}

#[tonic::async_trait]
impl SignalService for SignalServiceImpl {
    async fn get(&self, request: Request<GetRequest>) -> std::result::Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let mut signals = Vec::new();
        let mut success = true;
        let mut error_message = String::new();

        info!("Get request for paths: {:?}", req.paths);

        for path in req.paths {
            match self.vehicle_shadow.read().await.get_signal(path.clone()) {
                Ok(signal) => {
                    signals.push(convert_signal_to_proto(&signal));
                }
                Err(e) => {
                    error!("Failed to get signal {}: {}", path, e);
                    success = false;
                    error_message = format!("Failed to get signal {}: {}", path, e);
                    break;
                }
            }
        }

        Ok(Response::new(GetResponse {
            signals,
            success,
            error_message,
        }))
    }

    async fn set(&self, request: Request<SetRequest>) -> std::result::Result<Response<SetResponse>, Status> {
        let req = request.into_inner();
        let mut results = Vec::new();
        let mut success = true;
        let mut error_message = String::new();

        info!("Set request for {} signals", req.signals.len());

        for set_request in req.signals {
            let result = {
                let signal_result = self.vehicle_shadow.read().await.get_signal(set_request.path.clone());
                
                match signal_result {
                    Ok(mut signal) => {
                        // protoのStateをRustのStateに変換して設定
                        if let Some(proto_state) = &set_request.state {
                            apply_state_update(&mut signal.state, proto_state);
                        }

                        let set_result = self.vehicle_shadow.write().await.set_signal(signal.clone());
                        match set_result {
                            Ok(_) => {
                                // 値が変更されたので、購読者に通知
                                let response = SubscribeResponse {
                                    signal: Some(convert_signal_to_proto(&signal)),
                                    error_message: String::new(),
                                };
                                let subscription_manager = self.subscription_manager.clone();
                                let path = set_request.path.clone();
                                tokio::spawn(async move {
                                    let subscription_manager = subscription_manager.read().await;
                                    subscription_manager.notify(&path, response);
                                });

                                SetResult {
                                    path: set_request.path,
                                    success: true,
                                    error_message: String::new(),
                                }
                            },
                            Err(e) => {
                                error!("Failed to set signal {}: {}", set_request.path, e);
                                SetResult {
                                    path: set_request.path,
                                    success: false,
                                    error_message: format!("Failed to set signal: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Signal not found: {}", set_request.path);
                        SetResult {
                            path: set_request.path,
                            success: false,
                            error_message: format!("Signal not found: {}", e),
                        }
                    }
                }
            };

            if !result.success {
                success = false;
                error_message = result.error_message.clone();
            }

            results.push(result);
        }

        Ok(Response::new(SetResponse {
            results,
            success,
            error_message,
        }))
    }

    type SubscribeStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<SubscribeResponse, Status>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> std::result::Result<Response<Self::SubscribeStream>, Status> {
        let req = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        info!("Subscribe request for paths: {:?}", req.paths);

        // SubscriptionManagerに購読を登録
        for path in req.paths.clone() {
            let mut subscription_manager = self.subscription_manager.write().await;
            let mut subscription_rx = subscription_manager.subscribe(path.clone());
            
            // 現在の値を取得して送信
            if let Ok(signal) = self.vehicle_shadow.read().await.get_signal(path.clone()) {
                let response = SubscribeResponse {
                    signal: Some(convert_signal_to_proto(&signal)),
                    error_message: String::new(),
                };
                let _ = tx.send(Ok(response)).await;
            }
            
            // 購読ストリームからの通知を転送
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                while let Some(response) = subscription_rx.recv().await {
                    let _ = tx_clone.send(Ok(response)).await;
                }
            });
        }

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            rx,
        )))
    }

    async fn unsubscribe(
        &self,
        request: Request<UnsubscribeRequest>,
    ) -> std::result::Result<Response<UnsubscribeResponse>, Status> {
        let req = request.into_inner();
        let success = true;
        let error_message = String::new();

        info!("Unsubscribe request for paths: {:?}", req.paths);

        for path in req.paths {
            let mut subscription_manager = self.subscription_manager.write().await;
            subscription_manager.unsubscribe(&path);
        }

        Ok(Response::new(UnsubscribeResponse {
            success,
            error_message,
        }))
    }
}

// サーバーを起動する関数
pub async fn run_server(
    vehicle_shadow: VehicleShadow,
    addr: &str,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = addr.parse()?;
    let service = SignalServiceImpl::new(vehicle_shadow);

    info!("Starting gRPC server on {}", addr);

    tonic::transport::Server::builder()
        .add_service(SignalServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

// 部分的な更新を適用する関数
fn apply_state_update(current_state: &mut crate::signal::State, proto_state: &vehicle_shadow::State) {
    if let Some(ref proto_value) = proto_state.value {
        current_state.value = convert_proto_value_to_rust(proto_value);
    }
    if let Some(capability) = proto_state.capability {
        current_state.capability = capability;
    }
    if let Some(availability) = proto_state.availability {
        current_state.availability = availability;
    }
    if let Some(ref reserved) = proto_state.reserved {
        current_state.reserved = reserved.clone();
    }
}
