//! Request context

use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContext {
    pub storefront: celes::Country,
    pub query: Vec<(String, String)>,
}

/// Context container trait for filling out context in deserialized structs
pub trait ContextContainer {
    fn set_context(&mut self, context: Arc<RequestContext>);
}

impl<T: ContextContainer> ContextContainer for Option<T> {
    fn set_context(&mut self, context: Arc<RequestContext>) {
        if let Some(e) = self {
            e.set_context(context);
        }
    }
}

impl<T: ContextContainer> ContextContainer for Vec<T> {
    fn set_context(&mut self, context: Arc<RequestContext>) {
        for e in self {
            e.set_context(context.clone())
        }
    }
}

macro_rules! dummy_container_impl {
    ($($ty:ty),*) => {
        $(
            impl ContextContainer for $ty {
                fn set_context(&mut self, _: Arc<RequestContext>) {}
            }
        )*
    };
}

macro_rules! tuple_container_impl {
    ($($name:ident),*) => {
        impl<$($name:ContextContainer),*> ContextContainer for ($($name,)*)
        {
            fn set_context(&mut self, context: Arc<RequestContext>) {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                $($name.set_context(context.clone());)*
            }
        }
    };
}

dummy_container_impl!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool, String, &str);
tuple_container_impl!(A);
tuple_container_impl!(A, B);
tuple_container_impl!(A, B, C);
tuple_container_impl!(A, B, C, D);
tuple_container_impl!(A, B, C, D, E);
tuple_container_impl!(A, B, C, D, E, G);
tuple_container_impl!(A, B, C, D, E, G, H);
tuple_container_impl!(A, B, C, D, E, G, H, I);
tuple_container_impl!(A, B, C, D, E, G, H, I, J);
tuple_container_impl!(A, B, C, D, E, G, H, I, J, K);
tuple_container_impl!(A, B, C, D, E, G, H, I, J, K, L);
tuple_container_impl!(A, B, C, D, E, G, H, I, J, K, L, M);
