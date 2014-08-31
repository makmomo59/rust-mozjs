/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

/* automatically generated by rust-bindgen */

use libc;
use jsapi::*;
use jsfriendapi::JSJitInfo;
use jsval::JSVal;

pub static JS_STRUCTURED_CLONE_VERSION: u32 = 1;

pub struct ProxyTraps {
    pub getPropertyDescriptor: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, bool, *mut JSPropertyDescriptor) -> bool>,
    pub getOwnPropertyDescriptor: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, bool, *mut JSPropertyDescriptor) -> bool>,
    pub defineProperty: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut JSPropertyDescriptor) -> bool>,
    pub getOwnPropertyNames: *const u8, //XXX need a representation for AutoIdVector&
    pub delete_: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut bool) -> bool>,
    pub enumerate: *const u8, //XXX need a representation for AutoIdVector&

    pub has: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut bool) -> bool>,
    pub hasOwn: Option<extern "C" fn(*mut JSContext, *mut JSObject, jsid, *mut bool) -> bool>,
    pub get: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSObject, jsid, *mut JSVal) -> bool>,
    pub set: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSObject, jsid, bool, *mut JSVal) -> bool>,
    pub keys: *const u8, //XXX need a representation for AutoIdVector&
    pub iterate: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal) -> bool>,

    pub call: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal) -> bool>,
    pub construct: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal, *mut JSVal) -> bool>,
    pub nativeCall: *const u8, //XXX need a representation for IsAcceptableThis, NativeImpl, and CallArgs
    pub hasInstance: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSVal, *mut bool) -> bool>,
    pub typeOf: Option<extern "C" fn(*mut JSContext, *mut JSObject) -> uint>, //XXX JSType enum
    pub objectClassIs: Option<extern "C" fn(*mut JSObject, uint, *mut JSContext) -> bool>, //XXX ESClassValue enum
    pub obj_toString: Option<extern "C" fn(*mut JSContext, *mut JSObject) -> *mut JSString>,
    pub fun_toString: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint) -> *mut JSString>,
    //regexp_toShared: *u8,
    pub defaultValue: Option<extern "C" fn(*mut JSContext, *mut JSObject, uint, *mut JSVal) -> bool>, //XXX JSType enum
    pub iteratorNext: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSVal) -> bool>,
    pub finalize: Option<extern "C" fn(*mut JSFreeOp, *mut JSObject)>,
    pub getElementIfPresent: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut JSObject, u32, *mut JSVal, *mut bool) -> bool>,
    pub getPrototypeOf: Option<extern "C" fn(*mut JSContext, *mut JSObject, *mut *mut JSObject) -> bool>,
    pub trace: Option<extern "C" fn(*mut JSTracer, *mut JSObject)>,
}

pub static WRAPPER_ACTION_GET: libc::c_int = 0;
pub static WRAPPER_ACTION_SET: libc::c_int = 1;
pub static WRAPPER_ACTION_CALL: libc::c_int = 2;
pub static WRAPPER_ACTION_PUNCTURE: libc::c_int = 3;

#[link(name = "jsglue")]
extern { }

#[cfg(target_os = "android")]
#[link(name = "stdc++")]
extern { }

#[cfg(target_os = "android")]
#[link(name = "gcc")]
extern { }

extern {

//#[rust_stack]
pub fn RUST_JS_NumberValue(d: f64) -> JSVal;

//#[rust_stack]
pub fn CallJitPropertyOp(info: *const JSJitInfo, cx: *mut JSContext, thisObj: *mut JSObject, specializedThis: *mut libc::c_void, vp: *mut JSVal) -> JSBool;

//#[rust_stack]
pub fn CallJitMethodOp(info: *const JSJitInfo, cx: *mut JSContext, thisObj: *mut JSObject, specializedThis: *mut libc::c_void, argc: libc::c_uint, vp: *mut JSVal) -> JSBool;

//#[rust_stack]
pub fn RUST_FUNCTION_VALUE_TO_JITINFO(v: JSVal) -> *const JSJitInfo;

pub fn SetFunctionNativeReserved(fun: *mut JSObject, which: libc::size_t, val: *mut JSVal);
pub fn GetFunctionNativeReserved(fun: *mut JSObject, which: libc::size_t) -> *mut JSVal;

pub fn CreateProxyHandler(traps: *const ProxyTraps, extra: *const libc::c_void) -> *const libc::c_void;
pub fn CreateWrapperProxyHandler(traps: *const ProxyTraps) -> *const libc::c_void;
pub fn CreateCrossCompartmentSecurityWrapperProxyHandler(
    traps: *const ProxyTraps,
    enter: extern "C" fn(*mut JSContext, *mut JSObject, jsid, libc::c_int, *mut bool))
    -> *const libc::c_void;
pub fn NewProxyObject(cx: *mut JSContext, handler: *const libc::c_void, priv_: *const JSVal,
                      proto: *mut JSObject, parent: *mut JSObject, call: *mut JSObject,
                      construct: *mut JSObject) -> *mut JSObject;
pub fn WrapperNew(cx: *mut JSContext, parent: *mut JSObject, handler: *const libc::c_void) -> *mut JSObject;

pub fn GetProxyExtra(obj: *mut JSObject, slot: libc::c_uint) -> JSVal;
pub fn GetProxyPrivate(obj: *mut JSObject) -> JSVal;
pub fn SetProxyExtra(obj: *mut JSObject, slot: libc::c_uint, val: JSVal);

pub fn GetObjectProto(obj: *mut JSObject) -> *mut JSObject;
pub fn GetObjectParent(obj: *mut JSObject) -> *mut JSObject;

pub fn RUST_JSID_IS_INT(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_INT(id: jsid) -> libc::c_int;
pub fn RUST_JSID_IS_STRING(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_STRING(id: jsid) -> *mut JSString;

pub fn RUST_SET_JITINFO(func: *mut JSFunction, info: *const JSJitInfo);

pub fn RUST_INTERNED_STRING_TO_JSID(cx: *mut JSContext, str: *mut JSString) -> jsid;

pub fn DefineFunctionWithReserved(cx: *mut JSContext, obj: *mut JSObject,
                                  name: *const libc::c_char, call: JSNative, nargs: libc::c_uint,
                                  attrs: libc::c_uint) -> *mut JSObject;
pub fn GetObjectJSClass(obj: *mut JSObject) -> *mut JSClass;
pub fn RUST_js_GetErrorMessage(userRef: *mut libc::c_void, locale: *const libc::c_char,
                               errorNumber: libc::c_uint) -> *const JSErrorFormatString;
pub fn js_IsObjectProxyClass(obj: *mut JSObject) -> bool;
pub fn js_IsFunctionProxyClass(obj: *mut JSObject) -> bool;
pub fn IsProxyHandlerFamily(obj: *mut JSObject) -> bool;
pub fn GetProxyHandlerExtra(obj: *mut JSObject) -> *const libc::c_void;
pub fn GetProxyHandler(obj: *mut JSObject) -> *mut libc::c_void;
pub fn InvokeGetOwnPropertyDescriptor(handler: *mut libc::c_void, cx: *mut JSContext, proxy: *mut JSObject, id: jsid, set: bool, desc: *mut JSPropertyDescriptor) -> bool;
pub fn GetGlobalForObjectCrossCompartment(obj: *mut JSObject) -> *mut JSObject;
pub fn ReportError(cx: *mut JSContext, error: *const libc::c_char);
pub fn IsWrapper(obj: *mut JSObject) -> JSBool;
pub fn UnwrapObject(obj: *mut JSObject, stopAtOuter: JSBool, flags: *mut libc::c_uint) -> *mut JSObject;

pub fn GetObjectCompartment(object: *mut JSObject) -> *mut JSCompartment;
pub fn GetContextCompartment(cx: *mut JSContext) -> *mut JSCompartment;
pub fn GetCrossCompartmentWrapperSingleton() -> *const libc::c_void;
}
