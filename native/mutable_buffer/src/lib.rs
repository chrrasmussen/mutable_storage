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
        ("new", 1, buffer_new),
        ("get_byte", 2, buffer_get_byte),
        ("set_byte", 3, buffer_set_byte),
    ],
    Some(on_init)
}

struct Buffer {
    data: RwLock<Vec<u8>>,
}

fn on_init<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    rustler::resource_struct_init!(Buffer, env);
    true
}

fn buffer_new<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let buffer_size: usize = args[0].decode()?;

    let data = vec![0; buffer_size];
    let buffer = Buffer {
        data: RwLock::new(data),
    };

    Ok(ResourceArc::new(buffer).encode(env))
}

fn buffer_get_byte<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;
    let offset: usize = args[1].decode()?;

    let byte = resource.data.read().unwrap()[offset];
    Ok(byte.encode(env))
}

fn buffer_set_byte<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;
    let offset: usize = args[1].decode()?;
    let byte = args[2].decode()?;

    resource.data.write().unwrap()[offset] = byte;
    Ok(byte.encode(env))
}
