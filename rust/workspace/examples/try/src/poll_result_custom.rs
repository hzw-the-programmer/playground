use core::convert::Infallible;
use core::ops::{ControlFlow, FromResidual, Try};

#[derive(Debug)]
pub enum PollCustom<T> {
    Pending,
    Ready(T),
}

impl<T, E> Try for PollCustom<ResultCustom<T, E>> {
    type Output = PollCustom<T>;
    type Residual = ResultCustom<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        println!("PollCustom::from_output");
        output.map(ResultCustom::Ok)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        println!("PollCustom::branch");
        match self {
            PollCustom::Pending => ControlFlow::Continue(PollCustom::Pending),
            PollCustom::Ready(ResultCustom::Ok(t)) => ControlFlow::Continue(PollCustom::Ready(t)),
            PollCustom::Ready(ResultCustom::Err(e)) => ControlFlow::Break(ResultCustom::Err(e)),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<ResultCustom<Infallible, E>>
    for PollCustom<ResultCustom<T, F>>
{
    fn from_residual(residual: ResultCustom<Infallible, E>) -> Self {
        println!("PollCustom::from_residual");
        match residual {
            ResultCustom::Err(e) => PollCustom::Ready(ResultCustom::Err(From::from(e))),
            ResultCustom::Ok(_) => todo!(),
        }
    }
}

impl<T> PollCustom<T> {
    pub fn map<F, U>(self, f: F) -> PollCustom<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Pending => PollCustom::Pending,
            Self::Ready(t) => PollCustom::Ready(f(t)),
        }
    }
}

#[derive(Debug)]
pub enum ResultCustom<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Try for ResultCustom<T, E> {
    type Output = T;
    type Residual = ResultCustom<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        println!("ResultCustom::from_output");
        ResultCustom::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        println!("ResultCustom::branch");
        match self {
            ResultCustom::Ok(t) => ControlFlow::Continue(t),
            ResultCustom::Err(e) => ControlFlow::Break(ResultCustom::Err(e)),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<ResultCustom<Infallible, E>> for ResultCustom<T, F> {
    fn from_residual(residual: ResultCustom<Infallible, E>) -> Self {
        println!("ResultCustom::from_residual");
        match residual {
            ResultCustom::Err(e) => ResultCustom::Err(From::from(e)),
            ResultCustom::Ok(_) => todo!(),
        }
    }
}
