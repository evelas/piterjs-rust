#![deny(clippy::all)] 
use napi::{bindgen_prelude::*, JsObject, threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode}};
mod callback;

#[macro_use]
extern crate napi_derive;
 
#[napi]
pub struct NapiGrfResultBuffer {
    pub matches: String,
}
#[napi(
ts_args_type = "callback?: (stage: string, processed: number, total: number, label: string) => void",
ts_return_type = "Promise<string>"
)]
pub fn loading_files_info( env: Env, callback: Option<JsFunction>) -> Result<JsObject> {
  let mut callback_thread_safe: Option<ThreadsafeFunction<(String, u64, u64, String), ErrorStrategy::Fatal>> = None;

  if callback.is_some() {
    callback_thread_safe = Some(callback
        .unwrap()
        .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(String, u64, u64, String)>| {
          let mut v: Vec<JsObject> = Vec::new();
          v.push(ctx.env.create_string(ctx.value.0.as_str()).unwrap().coerce_to_object().unwrap());
          v.push(ctx.env.create_uint32(ctx.value.1 as u32).unwrap().coerce_to_object().unwrap());
          v.push(ctx.env.create_uint32(ctx.value.2 as u32).unwrap().coerce_to_object().unwrap());
          v.push(ctx.env.create_string(ctx.value.3.as_str()).unwrap().coerce_to_object().unwrap());
          Ok(v)
        }).unwrap());
  }
   env.execute_tokio_future(async {
    let result = callback::loading_files_with_progress(
      Box::new(move |s, p, t, l| {
          if callback_thread_safe.as_ref().is_some() {
              callback_thread_safe.as_ref().unwrap().call((s, p, t, l.unwrap_or("".to_string())), ThreadsafeFunctionCallMode::NonBlocking);
          }
      }),
     );
    Ok(result)
  }, |_env, data| Ok(data))

}

// Легкие примеры 
#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    print!("{}", a);
    print!("from println!");
    a + b
}

#[napi(object)]
pub struct Pet {
  pub name: String,
  pub kind: u32,
}
 
#[napi]
pub fn print_pet(pet: Pet) {
  println!("{}", pet.name);
}
 
#[napi]
pub fn create_cat() -> Pet {
  Pet {
    name: "dog".to_string(),
    kind: 2,
  }
}

 
