use rustler::{Encoder, Env, Error, ResourceArc, Term};
use std::io::Write;
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
        ("ioref_new", 1, ioref_new),
        ("ioref_get", 1, ioref_get),
        ("ioref_set", 2, ioref_set),
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
    Ok(atoms::ok().encode(env))
}


fn ioref_new<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let term_binary = args[0].to_binary();

    let data = term_binary.to_vec();
    let buffer = Buffer {
        data: RwLock::new(data),
    };

    Ok(ResourceArc::new(buffer).encode(env))
}

fn ioref_get<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;

    let term_binary = resource.data.read().unwrap();
    let (term, _size) = env.binary_to_term(term_binary.as_slice()).unwrap();

    Ok(term.encode(env))
}

fn ioref_set<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;
    let term_binary = args[1].to_binary();

    let mut data = resource.data.write().unwrap();
    data.clear();
    data.write(term_binary.as_slice()).unwrap();
    data.shrink_to_fit();

    Ok(atoms::ok().encode(env))
}
