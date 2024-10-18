use super::handler::Handler;
use std::marker::PhantomData;

pub trait Middleware<In, E: Handler<In>> {
    type Output: Handler<In>;
    fn transform(self, handler: E) -> Self::Output;
    fn chain<Out, M2, E3>(self, other: M2) -> JoinMiddleware<In, Out, Self::Output, Self, E, M2, E3>
    where
        Self: Sized + 'static,
        Self::Output: Handler<In, Out = Out>,
        In: Send + Sync + 'static,
        Out: Send + Sync + 'static,
        E: Handler<In, Out = Out>,
        M2: Middleware<In, E3, Output = E> + 'static,
        E3: Handler<In, Out = Out>,
    {
        JoinMiddleware::new(self, other)
    }
}

pub struct JoinMiddleware<In, Out, E1, M1, E2, M2, E3>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
    E1: Handler<In, Out = Out>,
    M1: Middleware<In, E2, Output = E1>,
    E2: Handler<In, Out = Out>,
    M2: Middleware<In, E3, Output = E2>,
    E3: Handler<In, Out = Out>,
{
    middleware1: M1,
    middleware2: M2,
    phantom1: PhantomData<In>,
    phantom2: PhantomData<Out>,
    phantom3: PhantomData<E1>,
    phantom4: PhantomData<E2>,
    phantom5: PhantomData<E3>,
}

impl<In, Out, E1, M1, E2, M2, E3> Middleware<In, E3> for JoinMiddleware<In, Out, E1, M1, E2, M2, E3>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
    E1: Handler<In, Out = Out>,
    M1: Middleware<In, E2, Output = E1>,
    E2: Handler<In, Out = Out>,
    M2: Middleware<In, E3, Output = E2>,
    E3: Handler<In, Out = Out>,
{
    type Output = E1;

    fn transform(self, handler: E3) -> Self::Output {
        self.middleware1
            .transform(self.middleware2.transform(handler))
    }
}

impl<In, Out, E1, M1, E2, M2, E3> JoinMiddleware<In, Out, E1, M1, E2, M2, E3>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
    E1: Handler<In, Out = Out>,
    M1: Middleware<In, E2, Output = E1>,
    E2: Handler<In, Out = Out>,
    M2: Middleware<In, E3, Output = E2>,
    E3: Handler<In, Out = Out>,
{
    pub fn new(middleware1: M1, middleware2: M2) -> JoinMiddleware<In, Out, E1, M1, E2, M2, E3> {
        JoinMiddleware {
            middleware1: middleware1,
            middleware2: middleware2,
            phantom1: PhantomData,
            phantom2: PhantomData,
            phantom3: PhantomData,
            phantom4: PhantomData,
            phantom5: PhantomData,
        }
    }
}
