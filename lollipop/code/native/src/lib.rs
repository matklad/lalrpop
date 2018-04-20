#[macro_use]
extern crate neon;
extern crate serde;
extern crate neon_serde;
extern crate lollipop;

use std::sync::Arc;
use std::ops;
use std::iter;

use neon::vm::{Call, JsResult, Lock, VmResult, This, Arguments};
use neon::mem::Handle;
use neon::scope::{RootScope};
use neon::js::class::{Class, JsClass};
use neon::js::{JsValue, JsString, JsFunction};

use serde::Serialize;
use serde::de::DeserializeOwned;

use neon_serde::{from_value, to_value};

use lollipop::{
    TextRange,
    ide
};

#[derive(Clone)]
pub struct FileHandle {
    inner: Arc<ide::File>
}

impl ops::Deref for FileHandle {
    type Target = ide::File;

    fn deref(&self) -> &ide::File {
        self.inner.deref()
    }
}

impl FileHandle {
    pub fn new(text: String) -> Self {
        FileHandle { inner: Arc::new(ide::File::new(text)) }
    }
}

declare_types! {
    pub class JsFileHandle for FileHandle {
        init(call) {
            let scope = call.scope;
            let file = match call.arguments.len() {
                1 => {
                    let text = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
                    FileHandle::new(text)
                }
                _ => unreachable!()
            };
            Ok(file)
        }
    }
}

fn parse(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let text = call.arguments.require(scope, 0)?.check::<JsString>()?;

    let class: Handle<JsClass<JsFileHandle>> = JsFileHandle::class(scope)?;
    let ctor: Handle<JsFunction<JsFileHandle>> = class.constructor(scope)?;
    let ctor_args = iter::once(text.upcast());
    let file = ctor.construct::<_, JsValue, _>(scope, ctor_args)?;
    Ok(file.upcast())
}

pub fn syntax_tree(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut handle = call.arguments.require(scope, 0)?.check::<JsFileHandle>()?;
    let result = handle.grab(|file| file.syntax_tree());
    ret(scope, result)
}

pub fn highlight(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut handle = call.arguments.require(scope, 0)?.check::<JsFileHandle>()?;
    let result = handle.grab(|file| file.highlight());
    ret(scope, result)
}

pub fn extend_selection(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut handle = call.arguments.require(scope, 0)?.check::<JsFileHandle>()?;
    let range: TextRange = arg(scope, &call.arguments, 1)?;
    let result = handle.grab(|file| file.extend_selection(range));
    ret(scope, result)
}


register_module!(m, {
    m.export("parse", parse)?;
    m.export("syntaxTree", syntax_tree)?;
    m.export("highlight", highlight)?;
    m.export("extendSelection", extend_selection)?;
    Ok(())
});

pub fn arg<A: DeserializeOwned + Send, T: This>(scope: &mut RootScope, args: &Arguments<T>, idx: i32) -> VmResult<A> {
    let arg = args.require(scope, idx)?;
    Ok(from_value(scope, arg)?)
}

pub fn ret<'j, T: Serialize>(scope: &mut RootScope<'j>, value: T) -> JsResult<'j, JsValue> {
    Ok(to_value(scope, &value)?)
}
