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
    buffer_raw_size,
    buffer_get_bits8,
    buffer_set_bits8,
    buffer_get_bits16,
    buffer_set_bits16,
    buffer_get_bits32,
    buffer_set_bits32,
    buffer_get_bits64,
    buffer_set_bits64,
    buffer_get_int32,
    buffer_set_int32,
    buffer_get_int64,
    buffer_set_int64,
    buffer_get_double,
    buffer_set_double,
    buffer_get_binary,
    buffer_set_binary,
    buffer_copy,
    term_new,
    term_get,
    term_set,
], load = load);

struct Buffer {
    data: RwLock<Vec<u8>>,
}

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
fn buffer_raw_size(resource: ResourceArc<Buffer>) -> usize {
    resource.data.read().unwrap().len()
}

#[rustler::nif]
fn buffer_get_bits8(resource: ResourceArc<Buffer>, offset: usize) -> u8 {
    resource.data.read().unwrap()[offset]
}

#[rustler::nif]
fn buffer_set_bits8(resource: ResourceArc<Buffer>, offset: usize, payload: u8) -> Atom {
    resource.data.write().unwrap()[offset] = payload;

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_bits16(resource: ResourceArc<Buffer>, offset: usize) -> u16 {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<u16>())]
        .try_into()
        .unwrap();
    u16::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_bits16(resource: ResourceArc<Buffer>, offset: usize, payload: u16) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<u16>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_bits32(resource: ResourceArc<Buffer>, offset: usize) -> u32 {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<u32>())]
        .try_into()
        .unwrap();
    u32::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_bits32(resource: ResourceArc<Buffer>, offset: usize, payload: u32) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<u32>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_bits64(resource: ResourceArc<Buffer>, offset: usize) -> u64 {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<u64>())]
        .try_into()
        .unwrap();
    u64::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_bits64(resource: ResourceArc<Buffer>, offset: usize, payload: u64) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<u64>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_int32(resource: ResourceArc<Buffer>, offset: usize) -> i32 {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<i32>())]
        .try_into()
        .unwrap();
    i32::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_int32(resource: ResourceArc<Buffer>, offset: usize, payload: i32) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<i32>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_int64(resource: ResourceArc<Buffer>, offset: usize) -> i64 {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<i64>())]
        .try_into()
        .unwrap();
    i64::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_int64(resource: ResourceArc<Buffer>, offset: usize, payload: i64) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<i64>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_double(resource: ResourceArc<Buffer>, offset: usize) -> f64 {
    let data = resource.data.read().unwrap();
    let bytes = data[offset..(offset + mem::size_of::<f64>())]
        .try_into()
        .unwrap();
    f64::from_ne_bytes(bytes)
}

#[rustler::nif]
fn buffer_set_double(resource: ResourceArc<Buffer>, offset: usize, payload: f64) -> Atom {
    let bytes = payload.to_ne_bytes();
    let mut data = resource.data.write().unwrap();
    data.splice(
        offset..(offset + mem::size_of::<f64>()),
        bytes.iter().cloned(),
    );

    atoms::ok()
}

#[rustler::nif]
fn buffer_get_binary(env: Env, resource: ResourceArc<Buffer>, offset: usize, length: usize) -> Binary {
    let data = resource.data.read().unwrap();
    let bytes = &data[offset..(offset + length)];

    let mut binary = OwnedBinary::new(length).unwrap();
    binary.as_mut_slice().write(bytes).unwrap();
    binary.release(env)
}

#[rustler::nif]
fn buffer_set_binary(resource: ResourceArc<Buffer>, offset: usize, payload: Binary) -> Atom {
    let bytes = payload.as_slice();
    let mut data = resource.data.write().unwrap();
    for i in 0..bytes.len() {
        data[offset + i] = bytes[i];
    }

    atoms::ok()
}

#[rustler::nif]
fn buffer_copy(src_resource: ResourceArc<Buffer>, src_offset: usize, length: usize, dest_resource: ResourceArc<Buffer>, dest_offset: usize) -> Atom {
    let src_data = src_resource.data.read().unwrap();
    let mut dest_data = dest_resource.data.write().unwrap();
    for i in 0..length {
        dest_data[dest_offset + i] = src_data[src_offset + i];
    }

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
