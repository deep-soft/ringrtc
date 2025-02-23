//
// Copyright 2019-2021 Signal Messenger, LLC
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::webrtc;
use crate::webrtc::ffi::media::{RffiAudioTrack, RffiVideoSource, RffiVideoTrack};
use crate::webrtc::ffi::peer_connection::RffiPeerConnection;
use crate::webrtc::ffi::peer_connection_observer::RffiPeerConnectionObserver;
#[cfg(feature = "injectable_network")]
use crate::webrtc::injectable_network::RffiInjectableNetwork;
use crate::webrtc::peer_connection_factory::{
    RffiAudioConfig, RffiIceServers, RffiPeerConnectionKind,
};
#[cfg(feature = "native")]
use std::os::raw::c_char;

/// Incomplete type for C++ PeerConnectionFactoryOwner.
#[repr(C)]
pub struct RffiPeerConnectionFactoryOwner {
    _private: [u8; 0],
}

impl webrtc::RefCounted for RffiPeerConnectionFactoryOwner {}

/// Incomplete type for C++ PeerConnectionFactoryInterface.
#[repr(C)]
pub struct RffiPeerConnectionFactoryInterface {
    _private: [u8; 0],
}

// See "class PeerConnectionFactoryInterface: public rtc::RefCountInterface"
// in webrtc/api/peer_connection_interface.h
impl webrtc::RefCounted for RffiPeerConnectionFactoryInterface {}

/// Incomplete type for C++ AudioDeviceModule.
#[repr(C)]
pub struct RffiAudioDeviceModule {
    _private: [u8; 0],
}

// See "class AudioDeviceModule : public rtc::RefCountInterface"
// in webrtc/modules/audio_device/include/audio_device.h
impl webrtc::RefCounted for RffiAudioDeviceModule {}

extern "C" {
    pub fn Rust_createPeerConnectionFactory(
        audio_config: RffiAudioConfig,
        use_injectable_network: bool,
    ) -> webrtc::ptr::OwnedRc<RffiPeerConnectionFactoryOwner>;
    pub fn Rust_createPeerConnectionFactoryWrapper(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryInterface>,
    ) -> webrtc::ptr::OwnedRc<RffiPeerConnectionFactoryOwner>;
    #[cfg(feature = "injectable_network")]
    // The injectable network will live as long as the PeerConnectionFactoryOwner.
    pub fn Rust_getInjectableNetwork(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
    ) -> webrtc::ptr::Borrowed<RffiInjectableNetwork>;
    #[allow(clippy::too_many_arguments)]
    pub fn Rust_createPeerConnection(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
        observer: webrtc::ptr::Borrowed<RffiPeerConnectionObserver>,
        kind: RffiPeerConnectionKind,
        audio_jitter_buffer_max_packets: isize,
        audio_jitter_buffer_max_target_delay_ms: isize,
        audio_rtcp_report_interval_ms: isize,
        ice_servers: RffiIceServers,
        outgoing_audio_track: webrtc::ptr::BorrowedRc<RffiAudioTrack>,
        outgoing_video_track: webrtc::ptr::BorrowedRc<RffiVideoTrack>,
    ) -> webrtc::ptr::OwnedRc<RffiPeerConnection>;
    pub fn Rust_createAudioTrack(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
    ) -> webrtc::ptr::OwnedRc<RffiAudioTrack>;
    pub fn Rust_createVideoSource() -> webrtc::ptr::OwnedRc<RffiVideoSource>;
    pub fn Rust_createVideoTrack(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
        source: webrtc::ptr::BorrowedRc<RffiVideoSource>,
    ) -> webrtc::ptr::OwnedRc<RffiVideoTrack>;
    #[cfg(feature = "native")]
    pub fn Rust_getAudioPlayoutDevices(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
    ) -> i16;
    #[cfg(feature = "native")]
    pub fn Rust_getAudioPlayoutDeviceName(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
        index: u16,
        name_out: *mut c_char,
        uuid_out: *mut c_char,
    ) -> i32;
    #[cfg(feature = "native")]
    pub fn Rust_setAudioPlayoutDevice(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
        index: u16,
    ) -> bool;
    #[cfg(feature = "native")]
    pub fn Rust_getAudioRecordingDevices(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
    ) -> i16;
    #[cfg(feature = "native")]
    pub fn Rust_getAudioRecordingDeviceName(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
        index: u16,
        name_out: *mut c_char,
        uuid_out: *mut c_char,
    ) -> i32;
    #[cfg(feature = "native")]
    pub fn Rust_setAudioRecordingDevice(
        factory: webrtc::ptr::BorrowedRc<RffiPeerConnectionFactoryOwner>,
        index: u16,
    ) -> bool;
}
