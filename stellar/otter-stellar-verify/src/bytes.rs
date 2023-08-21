use crate::{env::Env, token::BytesObject};

#[derive(Clone)]
pub struct Bytes {
    env: Env,
    obj: BytesObject,
}

impl Bytes {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: BytesObject) -> Self {
        Self { env, obj }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_object(&self) -> &BytesObject {
        &self.obj
    }

    pub fn to_object(&self) -> BytesObject {
        self.obj
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct BytesN<const N: usize>(Bytes);
