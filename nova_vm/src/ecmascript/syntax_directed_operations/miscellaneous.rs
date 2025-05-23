// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::hint::unreachable_unchecked;

use crate::{
    ecmascript::{
        builtins::ECMAScriptFunction,
        execution::{Agent, Environment, PrivateEnvironment},
        syntax_directed_operations::function_definitions::instantiate_ordinary_function_object,
    },
    engine::context::NoGcScope,
};
use oxc_ast::ast;

/// ### [8.6.1 Runtime Semantics: InstantiateFunctionObject](https://tc39.es/ecma262/#sec-runtime-semantics-instantiatefunctionobject)
///
/// The syntax-directed operation InstantiateFunctionObject takes arguments env
/// (an Environment Record) and privateEnv (a PrivateEnvironment Record or
/// null) and returns an ECMAScript function object.
pub(crate) fn instantiate_function_object<'a>(
    agent: &mut Agent,
    function: &ast::Function<'_>,
    env: Environment<'a>,
    private_env: Option<PrivateEnvironment<'a>>,
    gc: NoGcScope<'a, '_>,
) -> ECMAScriptFunction<'a> {
    // FunctionDeclaration :
    // function BindingIdentifier ( FormalParameters ) { FunctionBody }
    // function ( FormalParameters ) { FunctionBody }
    if !function.r#async && !function.generator {
        // 1. Return InstantiateOrdinaryFunctionObject of FunctionDeclaration with arguments env and privateEnv.
        instantiate_ordinary_function_object(agent, function, env, private_env, gc)
    } else
    // GeneratorDeclaration :
    // function * BindingIdentifier ( FormalParameters ) { GeneratorBody }
    // function * ( FormalParameters ) { GeneratorBody }
    if !function.r#async && function.generator {
        // 1. Return InstantiateGeneratorFunctionObject of GeneratorDeclaration with arguments env and privateEnv.
        instantiate_ordinary_function_object(agent, function, env, private_env, gc)
    } else
    // AsyncGeneratorDeclaration :
    // async function * BindingIdentifier ( FormalParameters ) { AsyncGeneratorBody }
    // async function * ( FormalParameters ) { AsyncGeneratorBody }
    if function.r#async && function.generator {
        // 1. Return InstantiateAsyncGeneratorFunctionObject of AsyncGeneratorDeclaration with arguments env and privateEnv.
        instantiate_ordinary_function_object(agent, function, env, private_env, gc)
    } else
    // AsyncFunctionDeclaration :
    // async function BindingIdentifier ( FormalParameters ) { AsyncFunctionBody }
    // async function ( FormalParameters ) { AsyncFunctionBody }
    if function.r#async && !function.generator {
        // 1. Return InstantiateAsyncFunctionObject of AsyncFunctionDeclaration with arguments env and privateEnv.
        instantiate_ordinary_function_object(agent, function, env, private_env, gc)
    } else {
        // SAFETY: Two boolean values, four branches.
        unsafe { unreachable_unchecked() };
    }
}
