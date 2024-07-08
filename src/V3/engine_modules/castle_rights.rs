//! Module for enumeration of the castle rights


use std::ops::{Index, IndexMut};


pub enum CastleRights {
    CWK,
    CWQ,
    CBK,
    CBQ,
}


/*
Used for indexing operations (container[index]) in immutable contexts.

container[index] is actually syntactic sugar for *container.index(index)

Allows let value = v[index] if the type of value implements Copy
*/
impl<T> Index<CastleRights> for [T] {
    type Output = T;

    fn index(&self, index: CastleRights) -> &Self::Output {
        match index {
            CastleRights::CWK => &self[0],
            CastleRights::CWQ => &self[1],
            CastleRights::CBK => &self[2],
            CastleRights::CBQ => &self[3],
        }
    }
}


/*
Used for indexing operations (container[index]) in mutable contexts.

container[index] is actually syntactic sugar for *container.index_mut(index)

Allows v[index] = value
*/
impl<T> IndexMut<CastleRights> for [T] {
    fn index_mut(&mut self, index: CastleRights) -> &mut Self::Output {
        match index {
            CastleRights::CWK => &mut self[0],
            CastleRights::CWQ => &mut self[1],
            CastleRights::CBK => &mut self[2],
            CastleRights::CBQ => &mut self[3],
        }
    }
}
