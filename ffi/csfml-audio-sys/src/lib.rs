#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate csfml_system_sys;
use csfml_system_sys::*;

extern "C" {
    pub fn sfListener_setGlobalVolume(volume: f32);
}
extern "C" {
    pub fn sfListener_getGlobalVolume() -> f32;
}
extern "C" {
    pub fn sfListener_setPosition(position: sfVector3f);
}
extern "C" {
    pub fn sfListener_getPosition() -> sfVector3f;
}
extern "C" {
    pub fn sfListener_setDirection(direction: sfVector3f);
}
extern "C" {
    pub fn sfListener_getDirection() -> sfVector3f;
}
extern "C" {
    pub fn sfListener_setUpVector(upVector: sfVector3f);
}
extern "C" {
    pub fn sfListener_getUpVector() -> sfVector3f;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfSoundStatus { sfStopped = 0, sfPaused = 1, sfPlaying = 2, }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfMusic([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSound([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSoundBuffer([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSoundBufferRecorder([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSoundRecorder([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSoundStream([u8; 0]);
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(::std::mem::size_of::<max_align_t>() , 32usize , concat ! (
               "Size of: " , stringify ! ( max_align_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce1 as * const _ as usize } , 0usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce2 as * const _ as usize } , 16usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce2 ) ));
}
impl Clone for max_align_t {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfMusic_createFromFile(filename: *const ::std::os::raw::c_char)
     -> *mut sfMusic;
}
extern "C" {
    pub fn sfMusic_createFromMemory(data: *const ::std::os::raw::c_void,
                                    sizeInBytes: usize) -> *mut sfMusic;
}
extern "C" {
    pub fn sfMusic_createFromStream(stream: *mut sfInputStream)
     -> *mut sfMusic;
}
extern "C" {
    pub fn sfMusic_destroy(music: *mut sfMusic);
}
extern "C" {
    pub fn sfMusic_setLoop(music: *mut sfMusic, loop_: sfBool);
}
extern "C" {
    pub fn sfMusic_getLoop(music: *const sfMusic) -> sfBool;
}
extern "C" {
    pub fn sfMusic_getDuration(music: *const sfMusic) -> sfTime;
}
extern "C" {
    pub fn sfMusic_play(music: *mut sfMusic);
}
extern "C" {
    pub fn sfMusic_pause(music: *mut sfMusic);
}
extern "C" {
    pub fn sfMusic_stop(music: *mut sfMusic);
}
extern "C" {
    pub fn sfMusic_getChannelCount(music: *const sfMusic)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfMusic_getSampleRate(music: *const sfMusic)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfMusic_getStatus(music: *const sfMusic) -> sfSoundStatus;
}
extern "C" {
    pub fn sfMusic_getPlayingOffset(music: *const sfMusic) -> sfTime;
}
extern "C" {
    pub fn sfMusic_setPitch(music: *mut sfMusic, pitch: f32);
}
extern "C" {
    pub fn sfMusic_setVolume(music: *mut sfMusic, volume: f32);
}
extern "C" {
    pub fn sfMusic_setPosition(music: *mut sfMusic, position: sfVector3f);
}
extern "C" {
    pub fn sfMusic_setRelativeToListener(music: *mut sfMusic,
                                         relative: sfBool);
}
extern "C" {
    pub fn sfMusic_setMinDistance(music: *mut sfMusic, distance: f32);
}
extern "C" {
    pub fn sfMusic_setAttenuation(music: *mut sfMusic, attenuation: f32);
}
extern "C" {
    pub fn sfMusic_setPlayingOffset(music: *mut sfMusic, timeOffset: sfTime);
}
extern "C" {
    pub fn sfMusic_getPitch(music: *const sfMusic) -> f32;
}
extern "C" {
    pub fn sfMusic_getVolume(music: *const sfMusic) -> f32;
}
extern "C" {
    pub fn sfMusic_getPosition(music: *const sfMusic) -> sfVector3f;
}
extern "C" {
    pub fn sfMusic_isRelativeToListener(music: *const sfMusic) -> sfBool;
}
extern "C" {
    pub fn sfMusic_getMinDistance(music: *const sfMusic) -> f32;
}
extern "C" {
    pub fn sfMusic_getAttenuation(music: *const sfMusic) -> f32;
}
extern "C" {
    pub fn sfSound_create() -> *mut sfSound;
}
extern "C" {
    pub fn sfSound_copy(sound: *const sfSound) -> *mut sfSound;
}
extern "C" {
    pub fn sfSound_destroy(sound: *mut sfSound);
}
extern "C" {
    pub fn sfSound_play(sound: *mut sfSound);
}
extern "C" {
    pub fn sfSound_pause(sound: *mut sfSound);
}
extern "C" {
    pub fn sfSound_stop(sound: *mut sfSound);
}
extern "C" {
    pub fn sfSound_setBuffer(sound: *mut sfSound,
                             buffer: *const sfSoundBuffer);
}
extern "C" {
    pub fn sfSound_getBuffer(sound: *const sfSound) -> *const sfSoundBuffer;
}
extern "C" {
    pub fn sfSound_setLoop(sound: *mut sfSound, loop_: sfBool);
}
extern "C" {
    pub fn sfSound_getLoop(sound: *const sfSound) -> sfBool;
}
extern "C" {
    pub fn sfSound_getStatus(sound: *const sfSound) -> sfSoundStatus;
}
extern "C" {
    pub fn sfSound_setPitch(sound: *mut sfSound, pitch: f32);
}
extern "C" {
    pub fn sfSound_setVolume(sound: *mut sfSound, volume: f32);
}
extern "C" {
    pub fn sfSound_setPosition(sound: *mut sfSound, position: sfVector3f);
}
extern "C" {
    pub fn sfSound_setRelativeToListener(sound: *mut sfSound,
                                         relative: sfBool);
}
extern "C" {
    pub fn sfSound_setMinDistance(sound: *mut sfSound, distance: f32);
}
extern "C" {
    pub fn sfSound_setAttenuation(sound: *mut sfSound, attenuation: f32);
}
extern "C" {
    pub fn sfSound_setPlayingOffset(sound: *mut sfSound, timeOffset: sfTime);
}
extern "C" {
    pub fn sfSound_getPitch(sound: *const sfSound) -> f32;
}
extern "C" {
    pub fn sfSound_getVolume(sound: *const sfSound) -> f32;
}
extern "C" {
    pub fn sfSound_getPosition(sound: *const sfSound) -> sfVector3f;
}
extern "C" {
    pub fn sfSound_isRelativeToListener(sound: *const sfSound) -> sfBool;
}
extern "C" {
    pub fn sfSound_getMinDistance(sound: *const sfSound) -> f32;
}
extern "C" {
    pub fn sfSound_getAttenuation(sound: *const sfSound) -> f32;
}
extern "C" {
    pub fn sfSound_getPlayingOffset(sound: *const sfSound) -> sfTime;
}
extern "C" {
    pub fn sfSoundBuffer_createFromFile(filename:
                                            *const ::std::os::raw::c_char)
     -> *mut sfSoundBuffer;
}
extern "C" {
    pub fn sfSoundBuffer_createFromMemory(data: *const ::std::os::raw::c_void,
                                          sizeInBytes: usize)
     -> *mut sfSoundBuffer;
}
extern "C" {
    pub fn sfSoundBuffer_createFromStream(stream: *mut sfInputStream)
     -> *mut sfSoundBuffer;
}
extern "C" {
    pub fn sfSoundBuffer_createFromSamples(samples: *const sfInt16,
                                           sampleCount: sfUint64,
                                           channelCount:
                                               ::std::os::raw::c_uint,
                                           sampleRate: ::std::os::raw::c_uint)
     -> *mut sfSoundBuffer;
}
extern "C" {
    pub fn sfSoundBuffer_copy(soundBuffer: *const sfSoundBuffer)
     -> *mut sfSoundBuffer;
}
extern "C" {
    pub fn sfSoundBuffer_destroy(soundBuffer: *mut sfSoundBuffer);
}
extern "C" {
    pub fn sfSoundBuffer_saveToFile(soundBuffer: *const sfSoundBuffer,
                                    filename: *const ::std::os::raw::c_char)
     -> sfBool;
}
extern "C" {
    pub fn sfSoundBuffer_getSamples(soundBuffer: *const sfSoundBuffer)
     -> *const sfInt16;
}
extern "C" {
    pub fn sfSoundBuffer_getSampleCount(soundBuffer: *const sfSoundBuffer)
     -> sfUint64;
}
extern "C" {
    pub fn sfSoundBuffer_getSampleRate(soundBuffer: *const sfSoundBuffer)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfSoundBuffer_getChannelCount(soundBuffer: *const sfSoundBuffer)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfSoundBuffer_getDuration(soundBuffer: *const sfSoundBuffer)
     -> sfTime;
}
extern "C" {
    pub fn sfSoundBufferRecorder_create() -> *mut sfSoundBufferRecorder;
}
extern "C" {
    pub fn sfSoundBufferRecorder_destroy(soundBufferRecorder:
                                             *mut sfSoundBufferRecorder);
}
extern "C" {
    pub fn sfSoundBufferRecorder_start(soundBufferRecorder:
                                           *mut sfSoundBufferRecorder,
                                       sampleRate: ::std::os::raw::c_uint)
     -> sfBool;
}
extern "C" {
    pub fn sfSoundBufferRecorder_stop(soundBufferRecorder:
                                          *mut sfSoundBufferRecorder);
}
extern "C" {
    pub fn sfSoundBufferRecorder_getSampleRate(soundBufferRecorder:
                                                   *const sfSoundBufferRecorder)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfSoundBufferRecorder_getBuffer(soundBufferRecorder:
                                               *const sfSoundBufferRecorder)
     -> *const sfSoundBuffer;
}
extern "C" {
    pub fn sfSoundBufferRecorder_setDevice(soundBufferRecorder:
                                               *mut sfSoundBufferRecorder,
                                           name:
                                               *const ::std::os::raw::c_char)
     -> sfBool;
}
extern "C" {
    pub fn sfSoundBufferRecorder_getDevice(soundBufferRecorder:
                                               *mut sfSoundBufferRecorder)
     -> *const ::std::os::raw::c_char;
}
pub type sfSoundRecorderStartCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut ::std::os::raw::c_void)
                              -> sfBool>;
pub type sfSoundRecorderProcessCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *const sfInt16,
                                               arg2: usize,
                                               arg3:
                                                   *mut ::std::os::raw::c_void)
                              -> sfBool>;
pub type sfSoundRecorderStopCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn sfSoundRecorder_create(onStart: sfSoundRecorderStartCallback,
                                  onProcess: sfSoundRecorderProcessCallback,
                                  onStop: sfSoundRecorderStopCallback,
                                  userData: *mut ::std::os::raw::c_void)
     -> *mut sfSoundRecorder;
}
extern "C" {
    pub fn sfSoundRecorder_destroy(soundRecorder: *mut sfSoundRecorder);
}
extern "C" {
    pub fn sfSoundRecorder_start(soundRecorder: *mut sfSoundRecorder,
                                 sampleRate: ::std::os::raw::c_uint)
     -> sfBool;
}
extern "C" {
    pub fn sfSoundRecorder_stop(soundRecorder: *mut sfSoundRecorder);
}
extern "C" {
    pub fn sfSoundRecorder_getSampleRate(soundRecorder:
                                             *const sfSoundRecorder)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfSoundRecorder_isAvailable() -> sfBool;
}
extern "C" {
    pub fn sfSoundRecorder_setProcessingInterval(soundRecorder:
                                                     *mut sfSoundRecorder,
                                                 interval: sfTime);
}
extern "C" {
    pub fn sfSoundRecorder_getAvailableDevices(count: *mut usize)
     -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sfSoundRecorder_getDefaultDevice()
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sfSoundRecorder_setDevice(soundRecorder: *mut sfSoundRecorder,
                                     name: *const ::std::os::raw::c_char)
     -> sfBool;
}
extern "C" {
    pub fn sfSoundRecorder_getDevice(soundRecorder: *mut sfSoundRecorder)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sfSoundRecorder_setChannelCount(soundRecorder:
                                               *mut sfSoundRecorder,
                                           channelCount:
                                               ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfSoundRecorder_getChannelCount(soundRecorder:
                                               *const sfSoundRecorder)
     -> ::std::os::raw::c_uint;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfSoundStreamChunk {
    pub samples: *mut sfInt16,
    pub sampleCount: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfSoundStreamChunk() {
    assert_eq!(::std::mem::size_of::<sfSoundStreamChunk>() , 16usize , concat
               ! ( "Size of: " , stringify ! ( sfSoundStreamChunk ) ));
    assert_eq! (::std::mem::align_of::<sfSoundStreamChunk>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( sfSoundStreamChunk ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSoundStreamChunk ) ) . samples as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSoundStreamChunk ) ,
                "::" , stringify ! ( samples ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSoundStreamChunk ) ) . sampleCount as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSoundStreamChunk ) ,
                "::" , stringify ! ( sampleCount ) ));
}
impl Clone for sfSoundStreamChunk {
    fn clone(&self) -> Self { *self }
}
pub type sfSoundStreamGetDataCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut sfSoundStreamChunk,
                                               arg2:
                                                   *mut ::std::os::raw::c_void)
                              -> sfBool>;
pub type sfSoundStreamSeekCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: sfTime,
                                               arg2:
                                                   *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn sfSoundStream_create(onGetData: sfSoundStreamGetDataCallback,
                                onSeek: sfSoundStreamSeekCallback,
                                channelCount: ::std::os::raw::c_uint,
                                sampleRate: ::std::os::raw::c_uint,
                                userData: *mut ::std::os::raw::c_void)
     -> *mut sfSoundStream;
}
extern "C" {
    pub fn sfSoundStream_destroy(soundStream: *mut sfSoundStream);
}
extern "C" {
    pub fn sfSoundStream_play(soundStream: *mut sfSoundStream);
}
extern "C" {
    pub fn sfSoundStream_pause(soundStream: *mut sfSoundStream);
}
extern "C" {
    pub fn sfSoundStream_stop(soundStream: *mut sfSoundStream);
}
extern "C" {
    pub fn sfSoundStream_getStatus(soundStream: *const sfSoundStream)
     -> sfSoundStatus;
}
extern "C" {
    pub fn sfSoundStream_getChannelCount(soundStream: *const sfSoundStream)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfSoundStream_getSampleRate(soundStream: *const sfSoundStream)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfSoundStream_setPitch(soundStream: *mut sfSoundStream,
                                  pitch: f32);
}
extern "C" {
    pub fn sfSoundStream_setVolume(soundStream: *mut sfSoundStream,
                                   volume: f32);
}
extern "C" {
    pub fn sfSoundStream_setPosition(soundStream: *mut sfSoundStream,
                                     position: sfVector3f);
}
extern "C" {
    pub fn sfSoundStream_setRelativeToListener(soundStream:
                                                   *mut sfSoundStream,
                                               relative: sfBool);
}
extern "C" {
    pub fn sfSoundStream_setMinDistance(soundStream: *mut sfSoundStream,
                                        distance: f32);
}
extern "C" {
    pub fn sfSoundStream_setAttenuation(soundStream: *mut sfSoundStream,
                                        attenuation: f32);
}
extern "C" {
    pub fn sfSoundStream_setPlayingOffset(soundStream: *mut sfSoundStream,
                                          timeOffset: sfTime);
}
extern "C" {
    pub fn sfSoundStream_setLoop(soundStream: *mut sfSoundStream,
                                 loop_: sfBool);
}
extern "C" {
    pub fn sfSoundStream_getPitch(soundStream: *const sfSoundStream) -> f32;
}
extern "C" {
    pub fn sfSoundStream_getVolume(soundStream: *const sfSoundStream) -> f32;
}
extern "C" {
    pub fn sfSoundStream_getPosition(soundStream: *const sfSoundStream)
     -> sfVector3f;
}
extern "C" {
    pub fn sfSoundStream_isRelativeToListener(soundStream:
                                                  *const sfSoundStream)
     -> sfBool;
}
extern "C" {
    pub fn sfSoundStream_getMinDistance(soundStream: *const sfSoundStream)
     -> f32;
}
extern "C" {
    pub fn sfSoundStream_getAttenuation(soundStream: *const sfSoundStream)
     -> f32;
}
extern "C" {
    pub fn sfSoundStream_getLoop(soundStream: *const sfSoundStream) -> sfBool;
}
extern "C" {
    pub fn sfSoundStream_getPlayingOffset(soundStream: *const sfSoundStream)
     -> sfTime;
}
