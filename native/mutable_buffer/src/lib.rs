use rustler::{Encoder, Env, Error, ResourceArc, Term};
use std::sync::RwLock;

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.MutableBuffer",
    [
        ("add", 2, add),
        ("new", 1, buffer_new),
        ("get_byte", 2, buffer_get_byte),
        ("set_byte", 3, buffer_set_byte),
    ],
    Some(on_init)
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}

struct Buffer {
    data: RwLock<Vec<u8>>,
}

fn on_init<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    // This macro will take care of defining and initializing a new resource
    // object type.
    rustler::resource_struct_init!(Buffer, env);
    true
}

fn buffer_new<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    // The NIF should have a single argument provided, namely
    // the size of the buffer we want to create.
    let buffer_size: usize = args[0].decode()?;

    // Create the actual buffer and initialize it with zeroes.
    let buffer = vec![0; buffer_size];
    // Make the actual struct
    let buffer_struct = Buffer {
        data: RwLock::new(buffer),
    };

    // Return it!
    Ok(ResourceArc::new(buffer_struct).encode(env))
}

fn buffer_get_byte<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let buffer: ResourceArc<Buffer> = args[0].decode()?;
    let offset: usize = args[1].decode()?;

    let byte = buffer.data.read().unwrap()[offset];
    Ok(byte.encode(env))
}

fn buffer_set_byte<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let buffer: ResourceArc<Buffer> = args[0].decode()?;
    let offset: usize = args[1].decode()?;
    let byte = args[2].decode()?;

    buffer.data.write().unwrap()[offset] = byte;
    Ok(byte.encode(env))
}
