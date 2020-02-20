//
// Copyright (C) 2019 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

//! Common error codes.

use crate::common::{CallId, DeviceId};

/// Platform independent error conditions.
#[derive(Fail, Debug)]
pub enum RingRtcError {
    // Project wide common error codes
    #[fail(display = "Mutex poisoned: {}", _0)]
    MutexPoisoned(String),
    #[fail(display = "Null pointer in: {}, var: {}", _0, _1)]
    NullPointer(String, String),
    #[fail(display = "Expecting non-none option value in: {}, var: {}", _0, _1)]
    OptionValueNotSet(String, String),

    // Call Manager error codes
    #[fail(display = "Active call already in progress, id: {}", _0)]
    CallAlreadyInProgress(CallId),
    #[fail(display = "No active call found")]
    NoActiveCall,
    #[fail(display = "CallID not found in call_map: {}", _0)]
    CallIdNotFound(CallId),
    #[fail(display = "No local DeviceID found")]
    LocalDeviceIdNotFound,
    #[fail(display = "Connection not found in connection_map: {}", _0)]
    ConnectionNotFound(DeviceId),
    #[fail(display = "Active device ID is already set, remote_device: {}", _0)]
    ActiveDeviceIdAlreadySet(DeviceId),
    #[fail(display = "Active Media Stream is already set, remote_device: {}", _0)]
    ActiveMediaStreamAlreadySet(DeviceId),
    #[fail(
        display = "Pending incoming call data is already set, remote_device: {}, offer: {}",
        _0, _1
    )]
    PendingCallAlreadySet(DeviceId, String),
    #[fail(
        display = "Application Connection is already set, remote_device: {}",
        _0
    )]
    AppConnectionAlreadySet(DeviceId),
    #[fail(display = "Application Call Context is already set, call_id: {}", _0)]
    AppCallContextAlreadySet(CallId),

    // WebRTC / C++ error codes
    #[fail(display = "Unable to create C++ PeerConnectionObserver")]
    CreatePeerConnectionObserver,
    #[fail(display = "Unable to create C++ DataChannel with label: {}", _0)]
    CreateDataChannel(String),
    #[fail(display = "Unable to create C++ DataChannelObserver")]
    CreateDataChannelObserver,

    // WebRTC / C++ session description error codes
    #[fail(
        display = "CreateSessionDescriptionObserver failure. error msg: {}, type: {}",
        _0, _1
    )]
    CreateSessionDescriptionObserver(String, i32),
    #[fail(
        display = "CreateSessionDescriptionObserver get result failure. error msg: {}",
        _0
    )]
    CreateSessionDescriptionObserverResult(String),
    #[fail(
        display = "SetSessionDescriptionObserver failure. error msg: {}, type: {}",
        _0, _1
    )]
    SetSessionDescriptionObserver(String, i32),
    #[fail(
        display = "SetSessionDescriptionObserver get result failure. error msg: {}",
        _0
    )]
    SetSessionDescriptionObserverResult(String),
    #[fail(display = "AddIceCandidate failure")]
    AddIceCandidate,

    // WebRTC / C++ offer / answer error codes
    #[fail(display = "Unable to retrieve sdp description from offer")]
    GetOfferDescription,
    #[fail(display = "Unable to convert sdp answer string to SessionDescriptionInterface object")]
    ConvertSdpAnswer,
    #[fail(display = "Unable to convert sdp offer string to SessionDescriptionInterface object")]
    ConvertSdpOffer,

    // DataChannel error codes
    #[fail(display = "Unable to send data channel message")]
    DataChannelSend,
    #[fail(display = "Data channel protocol error: {}", _0)]
    DataChannelProtocol(String),

    // IceGatherer error codes
    #[fail(display = "UseSharedIceGatherer failure")]
    UseIceGatherer,
    #[fail(display = "CreateIceGatherer failure")]
    CreateIceGatherer,

    // Misc error codes
    #[fail(display = "Event stream polling failed")]
    FsmStreamPoll,
}
