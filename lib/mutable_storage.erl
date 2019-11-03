-module(mutable_storage).
-export([buffer_new/1, buffer_raw_size/1]).
-export([buffer_get_bits8/2, buffer_set_bits8/3, buffer_get_bits16/2, buffer_set_bits16/3]).
-export([buffer_get_bits32/2, buffer_set_bits32/3, buffer_get_bits64/2, buffer_set_bits64/3]).
-export([buffer_get_int32/2, buffer_set_int32/3, buffer_get_int64/2, buffer_set_int64/3]).
-export([buffer_get_double/2, buffer_set_double/3]).
-export([buffer_get_binary/3, buffer_set_binary/3]).
-export([buffer_copy/5, buffer_resize/2]).
-export([term_new/1, term_get/1, term_set/2]).

-on_load(init/0).


init() ->
  ok = erlang:load_nif("./priv/native/libmutable_storage", 0).


buffer_new(_X) ->
  erlang:nif_error(nif_not_loaded).

buffer_raw_size(_X) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_bits8(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_bits8(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_bits16(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_bits16(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_bits32(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_bits32(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_bits64(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_bits64(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_int32(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_int32(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_int64(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_int64(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_double(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_double(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_get_binary(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_set_binary(_X, _Y, _Z) ->
  erlang:nif_error(nif_not_loaded).

buffer_copy(_X1, _X2, _X3, _X4, _X5) ->
  erlang:nif_error(nif_not_loaded).

buffer_resize(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).


term_new(_X) ->
  erlang:nif_error(nif_not_loaded).

term_get(_X) ->
  erlang:nif_error(nif_not_loaded).

term_set(_X, _Y) ->
  erlang:nif_error(nif_not_loaded).
