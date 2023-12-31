#![deny(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]

//! A simple and fast web server framework.

#[macro_use]
mod macros;

#[cfg(test)]
#[macro_use]
mod test_helper;

pub mod handler;

pub mod routing;

/// Utilities.
pub mod utils;

#[doc(inline)]
pub use http_kit::{
    header, Body, Endpoint, Error, Hook, Method, Middleware, Request, Response, Result, ResultExt,
    StatusCode, Uri,
};

#[doc(inline)]
pub use routing::{CreateRouteNode, Route};

pub use async_trait::async_trait;

/// Extract strong-typed object from your request.
pub mod extract;

/// Modify response or make a response,but in a strong-typed way.
pub mod responder;

pub mod middleware;
