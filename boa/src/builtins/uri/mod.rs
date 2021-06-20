//! This module implements the global `decodeURI` and encodURI functions.

use std::borrow::Borrow;

use crate::value::RcString;
use crate::{
    object::FunctionBuilder, property::Attribute, value::Value, BoaProfiler, Context, Result,
};
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};

type EncodeFuncType = fn(&RcString) -> Value;

// https://url.spec.whatwg.org/#fragment-percent-encode-set
const ENCODE_FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

#[derive(Debug, Clone, Copy)]
pub(crate) struct Uri;

impl Uri {
    const NAME: &'static str = "Uri";

    pub(crate) fn attribute() -> Attribute {
        Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE
    }

    pub(crate) fn init(context: &mut Context) -> (&'static str, Value, Attribute) {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let decode_uri = FunctionBuilder::new(context, Self::decode_uri)
            .name("decodeURI")
            .length(1)
            .callable(true)
            .constructable(false)
            .build();

        let encode_uri = FunctionBuilder::new(context, Self::encode_uri)
            .name("encodeURI")
            .length(1)
            .callable(true)
            .constructable(false)
            .build();

        context.register_global_property("decodeURI", decode_uri, Attribute::default());
        context.register_global_property("encodeURI", encode_uri, Attribute::default());

        let _global = context.global_object();

        (Self::NAME, Value::undefined(), Self::attribute())
    }

    pub(crate) fn handle_uri(args: &[Value], cb: EncodeFuncType) -> Result<Value> {
        Ok(args
            .get(0)
            .map(|arg_str| match arg_str {
                Value::String(ref arg_str_ref) => {
                    if arg_str_ref.is_empty() {
                        Value::string("")
                    } else {
                        cb(arg_str_ref)
                    }
                }
                _ => Value::Undefined,
            })
            .unwrap())
    }

    // The decodeURI() function decodes a Uniform Resource Identifier (URI) previously created by encodeURI() or by a similar routine.
    // More information:
    //  - [ECMAScript reference][spec]
    //  - [MDN documentation][mdn]
    //
    // [spec]: https://tc39.es/ecma262/#sec-decodeuri-encodeduri
    // [mdn]:  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURI
    pub(crate) fn decode_uri(_: &Value, args: &[Value], _context: &mut Context) -> Result<Value> {
        Self::handle_uri(args, |arg_str: &RcString| -> Value {
            Value::string(
                percent_decode(arg_str.as_bytes())
                    .decode_utf8()
                    .unwrap()
                    .borrow(),
            )
        })
    }

    // The encodeURI() function encodes a URI by replacing each instance of certain characters by one, two, three,
    // or four escape sequences representing the UTF-8 encoding of the character
    // (will only be four escape sequences for characters composed of two "surrogate" characters).
    //
    // More information:
    //  - [ECMAScript reference][spec]
    //  - [MDN documentation][mdn]
    //
    // [spec]: https://tc39.es/ecma262/#sec-encodeuri-uri
    // [mdn]:  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI
    pub(crate) fn encode_uri(_: &Value, args: &[Value], _context: &mut Context) -> Result<Value> {
        Self::handle_uri(args, |arg_str: &RcString| -> Value {
            Value::string(utf8_percent_encode(arg_str, ENCODE_FRAGMENT).to_string())
        })
    }
}
