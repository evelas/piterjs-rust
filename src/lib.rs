#![deny(clippy::all)] 
use napi::{bindgen_prelude::*, JsObject, JsNumber, threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode}, JsString};
mod rust_mod;

#[macro_use]
extern crate napi_derive;
 
#[napi(
  ts_args_type = "callback: (
    processed: number, 
    total: number, 
    fileIteration: number,
  ) => void",
  ts_return_type = "Promise<string>"
)]
pub fn loading_files_info_napi(
  env: Env, 
  callback: JsFunction
) -> Result<JsObject> {
  let callback_thread_safe: ThreadsafeFunction<(u64, u64, usize), ErrorStrategy::Fatal> = 
    callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(u64, u64, usize)>| {
      let mut v: Vec<JsNumber> = Vec::new();
      v.push(ctx.env.create_uint32(ctx.value.0 as u32).unwrap());
      v.push(ctx.env.create_uint32(ctx.value.1 as u32).unwrap());
      v.push(ctx.env.create_uint32(ctx.value.2 as u32).unwrap());
      Ok(v)
    })
    .unwrap()
  ;
  
  env.execute_tokio_future(async {
    let result = rust_mod::loading_files_with_progress(
      Box::new(move |p, t, f| {
        callback_thread_safe.call((p, t, f), ThreadsafeFunctionCallMode::NonBlocking);
      }),
    );
    Ok(result)
  }, |_env, data| Ok(data))
}

// Легкие примеры 
#[napi]
pub struct SimpleNapiExample {
    pub hello: String,
}


#[napi(constructor)]
pub struct AnimalWithDefaultConstructor {
  pub name: String,
  pub kind: u32,
}

#[napi(object)]
pub struct Pet {
  pub name: String,
  pub age: Option<u32>,
}
 
#[napi]
pub fn print_pet(pet: Pet) {
  println!("{}", pet.name);
}
 
#[napi]
pub fn create_cat() -> Pet {  
  Pet {
    name: "Vaska".to_string(),
    age: Some(2),
  }
}
 

 
