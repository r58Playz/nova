// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::ecmascript::builtins::Behaviour;
use crate::ecmascript::types::IntoValue;
use crate::engine::context::GcScope;
use crate::{
    ecmascript::{
        builders::ordinary_object_builder::OrdinaryObjectBuilder,
        builtins::{ArgumentsList, Builtin},
        execution::{Agent, JsResult, Realm},
        types::{BUILTIN_STRING_MEMORY, String, Value},
    },
    heap::WellKnownSymbolIndexes,
};

pub(crate) struct StringIteratorPrototype;

struct StringIteratorPrototypeNext;
impl Builtin for StringIteratorPrototypeNext {
    const NAME: String<'static> = BUILTIN_STRING_MEMORY.next;

    const LENGTH: u8 = 0;

    const BEHAVIOUR: Behaviour = Behaviour::Regular(StringIteratorPrototype::next);
}

impl StringIteratorPrototype {
    fn next<'gc>(
        _agent: &mut Agent,
        _this_value: Value,
        _arguments: ArgumentsList,
        _gc: GcScope<'gc, '_>,
    ) -> JsResult<'gc, Value<'gc>> {
        todo!();
    }

    pub(crate) fn create_intrinsic(agent: &mut Agent, realm: Realm<'static>) {
        let intrinsics = agent.get_realm_record_by_id(realm).intrinsics();
        let this = intrinsics.string_iterator_prototype();
        let iterator_prototype = intrinsics.iterator_prototype();

        OrdinaryObjectBuilder::new_intrinsic_object(agent, realm, this)
            .with_property_capacity(2)
            .with_prototype(iterator_prototype)
            .with_builtin_function_property::<StringIteratorPrototypeNext>()
            .with_property(|builder| {
                builder
                    .with_key(WellKnownSymbolIndexes::ToStringTag.into())
                    .with_value_readonly(BUILTIN_STRING_MEMORY.String_Iterator.into_value())
                    .with_enumerable(false)
                    .with_configurable(true)
                    .build()
            })
            .build();
    }
}
