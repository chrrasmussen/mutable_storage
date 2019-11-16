use rustler::{Encoder, Env, Error, ResourceArc, Term};
use std::convert::TryInto;
use std::io::Write;
use std::mem;
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
    "Elixir.MutableStorage",
    [
        ("buffer_new", 1, buffer_new),
        ("buffer_get_byte", 2, buffer_get_byte),
        ("buffer_set_byte", 3, buffer_set_byte),
        ("buffer_get_int", 2, buffer_get_int),
        ("buffer_set_int", 3, buffer_set_int),
        ("term_new", 1, term_new),
        ("term_get", 1, term_get),
        ("term_set", 2, term_set),
    ],
    Some(on_init)
}

struct Buffer {
    data: RwLock<Vec<u8>>,
}

type IntType = i64;

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

fn buffer_get_int<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;
    let offset: usize = args[1].decode()?;

    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<IntType>())]
        .try_into()
        .unwrap();
    let int = IntType::from_ne_bytes(bytes);

    Ok(int.encode(env))
}

fn buffer_set_int<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;
    let offset: usize = args[1].decode()?;
    let int: IntType = args[2].decode()?;

    let bytes = int.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<IntType>()),
        bytes.iter().cloned(),
    );

    Ok(atoms::ok().encode(env))
}

fn term_new<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let term_binary = args[0].to_binary();

    let data = term_binary.to_vec();
    let buffer = Buffer {
        data: RwLock::new(data),
    };

    Ok(ResourceArc::new(buffer).encode(env))
}

fn term_get<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;

    let term_binary = resource.data.read().unwrap();
    let (term, _size) = env.binary_to_term(term_binary.as_slice()).unwrap();

    Ok(term.encode(env))
}

fn term_set<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Buffer> = args[0].decode()?;
    let term_binary = args[1].to_binary();

    let mut data = resource.data.write().unwrap();
    data.clear();
    data.write(term_binary.as_slice()).unwrap();
    data.shrink_to_fit();

    Ok(atoms::ok().encode(env))
}
