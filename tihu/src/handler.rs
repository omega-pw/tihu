use async_trait::async_trait;
use std::future::Future;

#[async_trait]
pub trait Handler<In>: Send + Sync + 'static {
    type Out;
    async fn handle(&self, input: In) -> Self::Out;
    fn map_output<NewOut, M>(self, output_mapper: M) -> OutputMapperHandler<Self, M>
    where
        Self: Sized,
        M: Fn(Self::Out) -> NewOut + Send + Sync + 'static,
    {
        OutputMapperHandler {
            handler: self,
            output_mapper: output_mapper,
        }
    }
}

#[async_trait]
impl<In, Out, FutRet, T> Handler<In> for T
where
    In: Send + 'static,
    Out: Send + 'static,
    FutRet: Future<Output = Out> + Send + 'static,
    T: Fn(In) -> FutRet + Send + Sync + 'static,
{
    type Out = Out;
    async fn handle(&self, input: In) -> Out {
        self(input).await
    }
}

pub struct Mapper<M>(pub M);

impl<M> Mapper<M> {
    pub fn chain<H, In, Out>(self, handler: H) -> InputMapperHandler<H, M>
    where
        M: Fn(In) -> Out + Send + Sync + 'static,
        H: Handler<Out>,
    {
        InputMapperHandler {
            input_mapper: self.0,
            handler: handler,
        }
    }
}

pub struct InputMapperHandler<H, M> {
    input_mapper: M,
    handler: H,
}

#[async_trait]
impl<In, NewIn, H, M> Handler<In> for InputMapperHandler<H, M>
where
    In: Send + 'static,
    NewIn: Send + 'static,
    H: Handler<NewIn>,
    M: Fn(In) -> NewIn + Send + Sync + 'static,
{
    type Out = H::Out;
    async fn handle(&self, input: In) -> H::Out {
        let new_input = (self.input_mapper)(input);
        self.handler.handle(new_input).await
    }
}

pub struct OutputMapperHandler<H, M> {
    handler: H,
    output_mapper: M,
}

#[async_trait]
impl<In, H, NewOut, M> Handler<In> for OutputMapperHandler<H, M>
where
    In: Send + 'static,
    NewOut: Send + 'static,
    H: Handler<In>,
    M: Fn(H::Out) -> NewOut + Send + Sync + 'static,
{
    type Out = NewOut;
    async fn handle(&self, input: In) -> NewOut {
        let output = self.handler.handle(input).await;
        (self.output_mapper)(output)
    }
}
