use rustler::{Atom, Binary, Env, OwnedBinary, ResourceArc, Term};
use std::convert::TryInto;
use std::io::Write;
use std::mem;
use std::sync::RwLock;

mod atoms {
    rustler::atoms! {
        ok,
    }
}

rustler::init!("Elixir.MutableStorage", [
    buffer_new,
    buffer_get_byte,
    buffer_set_byte,
    buffer_get_int,
    buffer_set_int,
    buffer_get_double,
    buffer_set_double,
    buffer_get_binary,
    buffer_set_binary,
    term_new,
    term_get,
    term_set,
], load = load);

struct Buffer {
    data: RwLock<Vec<u8>>,
}

type IntType = i64;
type DoubleType = f64;

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(Buffer, env);
    true
}

#[rustler::nif]
fn buffer_new(buffer_size: usize) -> ResourceArc<Buffer> {
    let data = vec![0; buffer_size];
    ResourceArc::new(Buffer {
        data: RwLock::new(data),
    })
}

#[rustler::nif]
fn buffer_get_byte(resource: ResourceArc<Buffer>, offset: usize) -> u8 {
    resource.data.read().unwrap()[offset]
}

#[rustler::nif]
fn buffer_set_byte(resource: ResourceArc<Buffer>, offset: usize, payload: u8) -> Atom {
    resource.data.write().unwrap()[offset] = payload;

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_int(resource: ResourceArc<Buffer>, offset: usize) -> IntType {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<IntType>())]
        .try_into()
        .unwrap();
    IntType::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_int(resource: ResourceArc<Buffer>, offset: usize, payload: IntType) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<IntType>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_double(resource: ResourceArc<Buffer>, offset: usize) -> DoubleType {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<DoubleType>())]
        .try_into()
        .unwrap();
    DoubleType::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_double(resource: ResourceArc<Buffer>, offset: usize, payload: DoubleType) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<DoubleType>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_binary(env: Env, resource: ResourceArc<Buffer>, offset: usize, length: usize) -> Binary {
    let mut binary = OwnedBinary::new(length).unwrap();
    let data = resource.data.read().unwrap();
    let bytes = &data[offset..(offset + length)];
    binary.as_mut_slice().write(bytes).unwrap();
    binary.release(env)
}

#[rustler::nif]
fn buffer_set_binary(resource: ResourceArc<Buffer>, offset: usize, payload: Binary) -> Atom {
    let bytes = payload.as_slice();
    let mut data = resource.data.write().unwrap();
    data.splice(offset..(offset + bytes.len()), bytes.iter().cloned());

    atoms::ok()
}

#[rustler::nif]
fn term_new(term: Term) -> ResourceArc<Buffer> {
    let data = term.to_binary().to_vec();
    let buffer = Buffer {
        data: RwLock::new(data),
    };

    ResourceArc::new(buffer)
}

#[rustler::nif]
fn term_get(env: Env, resource: ResourceArc<Buffer>) -> Term {
    let term_binary = resource.data.read().unwrap();
    let (term, _size) = env.binary_to_term(term_binary.as_slice()).unwrap();
    term
}

#[rustler::nif]
fn term_set(resource: ResourceArc<Buffer>, term: Term) -> Atom {
    let mut data = resource.data.write().unwrap();
    data.clear();
    data.write(term.to_binary().as_slice()).unwrap();
    data.shrink_to_fit();

    atoms::ok()
}
