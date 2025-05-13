use std::{future::Future, os::fd::AsFd, sync::Arc};

use async_io_mini::Async;

pub struct ReadableOwned<T: AsFd> {
    inner: Arc<Async<T>>,
}

impl<T: AsFd> ReadableOwned<T> {
    pub fn new(inner: Arc<Async<T>>) -> Self {
        Self { inner }
    }
}

impl<T: AsFd> Future for ReadableOwned<T> {
    type Output = std::io::Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.inner.poll_readable(cx)
    }
}

pub struct WritableOwned<T: AsFd> {
    inner: Arc<Async<T>>,
}

impl<T: AsFd> WritableOwned<T> {
    pub fn new(inner: Arc<Async<T>>) -> Self {
        Self { inner }
    }
}

impl<T: AsFd> Future for WritableOwned<T> {
    type Output = std::io::Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.inner.poll_writable(cx)
    }
}
