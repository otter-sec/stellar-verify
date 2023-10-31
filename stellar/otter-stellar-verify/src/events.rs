use crate::{Env, IntoVal, Val, Vec};

#[derive(Clone)]
pub struct Events(Env);

pub trait Topics: IntoVal<Env, Vec<Val>> {}

// macro_rules! impl_topics_for_tuple {
//     ( $($typ:ident $idx:tt)* ) => {
//         impl<$($typ),*> Topics for ($($typ,)*)
//         where
//             $($typ: IntoVal<Env, Val>),*
//         {
//         }
//     };
// }

// // 0 topics
// impl Topics for () {}
// // 1-4 topics
// impl_topics_for_tuple! { T0 0 }
// impl_topics_for_tuple! { T0 0 T1 1 }
// impl_topics_for_tuple! { T0 0 T1 1 T2 2 }
// impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 }

impl Events {
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    pub(crate) fn new(env: &Env) -> Events {
        Events(env.clone())
    }

    pub fn publish<T, D>(&self, _topics: T, _data: D) {}
}
