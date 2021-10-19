/*
 *
 * Copyright 2015, Google Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

// Because rust's union implementation is unstable and possibly buggy
// (rust-lang/rust#32836),
// so we need to wrap the type and expose more safer interfaces.

#include <grpc/byte_buffer_reader.h>
#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/support/alloc.h>
#include <grpc/support/log.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/string_util.h>
#include <grpc/support/thd_id.h>

#ifdef GRPC_SYS_SECURE
#include <grpc/grpc_security.h>
#endif

#include <string.h>

#ifdef GPR_WINDOWS
#define GPR_EXPORT extern "C" __declspec(dllexport)
#define GPR_CALLTYPE __cdecl
#endif

#ifndef GPR_EXPORT
#define GPR_EXPORT extern "C"
#endif

#ifndef GPR_CALLTYPE
#define GPR_CALLTYPE
#endif

grpc_byte_buffer* string_to_byte_buffer(const char* buffer, size_t len) {
  grpc_slice slice = grpc_slice_from_copied_buffer(buffer, len);
  grpc_byte_buffer* bb = grpc_raw_byte_buffer_create(&slice, 1);
  grpc_slice_unref(slice);
  return bb;
}

/*
 * Helper to maintain lifetime of batch op inputs and store batch op outputs.
 */
typedef struct grpcwrap_batch_context {
  grpc_metadata_array send_initial_metadata;
  grpc_byte_buffer* send_message;
  struct {
    grpc_metadata_array trailing_metadata;
  } send_status_from_server;
  grpc_metadata_array recv_initial_metadata;
  grpc_byte_buffer* recv_message;
  struct {
    grpc_metadata_array trailing_metadata;
    grpc_status_code status;
    grpc_slice status_details;
  } recv_status_on_client;
  int recv_close_on_server_cancelled;
} grpcwrap_batch_context;

GPR_EXPORT grpcwrap_batch_context* GPR_CALLTYPE
grpcwrap_batch_context_create() {
  auto* ctx =
      (grpcwrap_batch_context*)gpr_malloc(sizeof(grpcwrap_batch_context));
  memset(ctx, 0, sizeof(grpcwrap_batch_context));
  return ctx;
}

typedef struct {
  grpc_call* call;
  grpc_call_details call_details;
  grpc_metadata_array request_metadata;
} grpcwrap_request_call_context;

GPR_EXPORT grpcwrap_request_call_context* GPR_CALLTYPE
grpcwrap_request_call_context_create() {
  auto* ctx = (grpcwrap_request_call_context*)gpr_malloc(
      sizeof(grpcwrap_request_call_context));
  memset(ctx, 0, sizeof(grpcwrap_request_call_context));
  return ctx;
}

/*
 * Destroys array->metadata.
 * The array pointer itself is not freed.
 */
void grpcwrap_metadata_array_destroy_metadata_only(grpc_metadata_array* array) {
  gpr_free(array->metadata);
}

/*
 * Destroys keys, values and array->metadata.
 * The array pointer itself is not freed.
 */
void grpcwrap_metadata_array_destroy_metadata_including_entries(
    grpc_metadata_array* array) {
  size_t i;
  if (array->metadata) {
    for (i = 0; i < array->count; i++) {
      grpc_slice_unref(array->metadata[i].key);
      grpc_slice_unref(array->metadata[i].value);
    }
  }
  gpr_free(array->metadata);
}

/*
 * Fully destroys the metadata array.
 */
GPR_EXPORT void GPR_CALLTYPE
grpcwrap_metadata_array_destroy_full(grpc_metadata_array* array) {
  if (!array) {
    return;
  }
  grpcwrap_metadata_array_destroy_metadata_including_entries(array);
  gpr_free(array);
}

/*
 * Allocate metadata array with given capacity.
 */
GPR_EXPORT void GPR_CALLTYPE
grpcwrap_metadata_array_init(grpc_metadata_array* array, size_t capacity) {
  array->count = 0;
  array->capacity = capacity;
  if (!capacity) {
    array->metadata = nullptr;
    return;
  }

  auto* arr = (grpc_metadata*)gpr_malloc(sizeof(grpc_metadata) * capacity);
  memset(arr, 0, sizeof(grpc_metadata) * capacity);
  array->metadata = arr;
}

GPR_EXPORT void GPR_CALLTYPE grpcwrap_metadata_array_add(
    grpc_metadata_array* array, const char* key, size_t key_length,
    const char* value, size_t value_length) {
  GPR_ASSERT(array->count <= array->capacity);
  size_t i = array->count;
  if (i == array->capacity) {
    array->capacity = array->capacity ? array->capacity * 2 : 4;
    array->metadata = (grpc_metadata*)gpr_realloc(
        array->metadata, array->capacity * sizeof(grpc_metadata));
    memset(array->metadata + i, 0,
           sizeof(grpc_metadata) * (array->capacity - i));
  }
  array->metadata[i].key = grpc_slice_from_copied_buffer(key, key_length);
  array->metadata[i].value = grpc_slice_from_copied_buffer(value, value_length);
  array->count++;
}

GPR_EXPORT const char* GPR_CALLTYPE grpcwrap_metadata_array_get_key(
    const grpc_metadata_array* array, size_t index, size_t* key_length) {
  GPR_ASSERT(index < array->count);
  *key_length = GRPC_SLICE_LENGTH(array->metadata[index].key);
  return (char*)GRPC_SLICE_START_PTR(array->metadata[index].key);
}

GPR_EXPORT const char* GPR_CALLTYPE grpcwrap_metadata_array_get_value(
    const grpc_metadata_array* array, size_t index, size_t* value_length) {
  GPR_ASSERT(index < array->count);
  *value_length = GRPC_SLICE_LENGTH(array->metadata[index].value);
  return (char*)GRPC_SLICE_START_PTR(array->metadata[index].value);
}

GPR_EXPORT void GPR_CALLTYPE
grpcwrap_metadata_array_cleanup(grpc_metadata_array* array) {
  grpcwrap_metadata_array_destroy_metadata_including_entries(array);
}

GPR_EXPORT void GPR_CALLTYPE
grpcwrap_metadata_array_shrink_to_fit(grpc_metadata_array* array) {
  GPR_ASSERT(array->count <= array->capacity);
  if (array->count == array->capacity) {
    return;
  }
  if (array->count) {
    array->metadata = (grpc_metadata*)gpr_realloc(
        array->metadata, array->count * sizeof(grpc_metadata));
    array->capacity = array->count;
  } else {
    grpcwrap_metadata_array_cleanup(array);
    array->capacity = 0;
    array->metadata = nullptr;
  }
}

/* Move contents of metadata array */
void grpcwrap_metadata_array_move(grpc_metadata_array* dest,
                                  grpc_metadata_array* src) {
  if (!src) {
    dest->capacity = 0;
    dest->count = 0;
    dest->metadata = nullptr;
    return;
  }

  dest->capacity = src->capacity;
  dest->count = src->count;
  dest->metadata = src->metadata;

  src->capacity = 0;
  src->count = 0;
  src->metadata = nullptr;
}

GPR_EXPORT void GPR_CALLTYPE
grpcwrap_batch_context_destroy(grpcwrap_batch_context* ctx) {
  if (!ctx) {
    return;
  }
  grpcwrap_metadata_array_destroy_metadata_including_entries(
      &(ctx->send_initial_metadata));

  grpc_byte_buffer_destroy(ctx->send_message);

  grpcwrap_metadata_array_destroy_metadata_including_entries(
      &(ctx->send_status_from_server.trailing_metadata));

  grpcwrap_metadata_array_destroy_metadata_only(&(ctx->recv_initial_metadata));

  grpc_byte_buffer_destroy(ctx->recv_message);

  grpcwrap_metadata_array_destroy_metadata_only(
      &(ctx->recv_status_on_client.trailing_metadata));
  grpc_slice_unref(ctx->recv_status_on_client.status_details);

  gpr_free(ctx);
}

GPR_EXPORT void GPR_CALLTYPE
grpcwrap_request_call_context_destroy(grpcwrap_request_call_context* ctx) {
  if (!ctx) {
    return;
  }

  if (ctx->call) {
    grpc_call_unref(ctx->call);
  }

  grpc_call_details_destroy(&(ctx->call_details));
  grpcwrap_metadata_array_destroy_metadata_only(&(ctx->request_metadata));

  gpr_free(ctx);
}

GPR_EXPORT const grpc_metadata_array* GPR_CALLTYPE
grpcwrap_batch_context_recv_initial_metadata(
    const grpcwrap_batch_context* ctx) {
  return &(ctx->recv_initial_metadata);
}

GPR_EXPORT const char* GPR_CALLTYPE
grpcwrap_slice_raw_offset(const grpc_slice* slice, size_t offset, size_t* len) {
  *len = GRPC_SLICE_LENGTH(*slice) - offset;
  return (const char*)(GRPC_SLICE_START_PTR(*slice)) + offset;
}

GPR_EXPORT grpc_slice GPR_CALLTYPE
grpcwrap_slice_copy(const grpc_slice* slice) {
  return grpc_slice_copy(*slice);
}

GPR_EXPORT void GPR_CALLTYPE grpcwrap_slice_unref(const grpc_slice* slice) {
  grpc_slice_unref(*slice);
}

GPR_EXPORT grpc_slice GPR_CALLTYPE grpcwrap_slice_ref(const grpc_slice* slice) {
  return grpc_slice_ref(*slice);
}

GPR_EXPORT size_t GPR_CALLTYPE grpcwrap_slice_length(const grpc_slice* slice) {
  return GRPC_SLICE_LENGTH(*slice);
}

GPR_EXPORT grpc_byte_buffer* GPR_CALLTYPE
grpcwrap_batch_context_take_recv_message(grpcwrap_batch_context* ctx) {
  grpc_byte_buffer* buf = nullptr;
  if (ctx->recv_message) {
    buf = ctx->recv_message;
    ctx->recv_message = nullptr;
  }
  return buf;
}

GPR_EXPORT grpc_status_code GPR_CALLTYPE
grpcwrap_batch_context_recv_status_on_client_status(
    const grpcwrap_batch_context* ctx) {
  return ctx->recv_status_on_client.status;
}

GPR_EXPORT const char* GPR_CALLTYPE
grpcwrap_batch_context_recv_status_on_client_details(
    const grpcwrap_batch_context* ctx, size_t* details_length) {
  *details_length =
      GRPC_SLICE_LENGTH(ctx->recv_status_on_client.status_details);
  return (char*)GRPC_SLICE_START_PTR(ctx->recv_status_on_client.status_details);
}

GPR_EXPORT const grpc_metadata_array* GPR_CALLTYPE
grpcwrap_batch_context_recv_status_on_client_trailing_metadata(
    const grpcwrap_batch_context* ctx) {
  return &(ctx->recv_status_on_client.trailing_metadata);
}

GPR_EXPORT grpc_call* GPR_CALLTYPE
grpcwrap_request_call_context_ref_call(grpcwrap_request_call_context* ctx) {
  grpc_call* call = ctx->call;
  grpc_call_ref(call);
  return call;
}

GPR_EXPORT grpc_call* GPR_CALLTYPE
grpcwrap_request_call_context_get_call(grpcwrap_request_call_context* ctx) {
  return ctx->call;
}

GPR_EXPORT const char* GPR_CALLTYPE grpcwrap_request_call_context_method(
    const grpcwrap_request_call_context* ctx, size_t* method_length) {
  *method_length = GRPC_SLICE_LENGTH(ctx->call_details.method);
  return (char*)GRPC_SLICE_START_PTR(ctx->call_details.method);
}

GPR_EXPORT const char* GPR_CALLTYPE grpcwrap_request_call_context_host(
    const grpcwrap_request_call_context* ctx, size_t* host_length) {
  *host_length = GRPC_SLICE_LENGTH(ctx->call_details.host);
  return (char*)GRPC_SLICE_START_PTR(ctx->call_details.host);
}

GPR_EXPORT gpr_timespec GPR_CALLTYPE grpcwrap_request_call_context_deadline(
    const grpcwrap_request_call_context* ctx) {
  return ctx->call_details.deadline;
}

GPR_EXPORT const grpc_metadata_array* GPR_CALLTYPE
grpcwrap_request_call_context_metadata_array(
    const grpcwrap_request_call_context* ctx) {
  return &(ctx->request_metadata);
}

GPR_EXPORT int32_t GPR_CALLTYPE
grpcwrap_batch_context_recv_close_on_server_cancelled(
    const grpcwrap_batch_context* ctx) {
  return (int32_t)ctx->recv_close_on_server_cancelled;
}

/* Channel */

GPR_EXPORT grpc_call* GPR_CALLTYPE grpcwrap_channel_create_call(
    grpc_channel* channel, grpc_call* parent_call, uint32_t propagation_mask,
    grpc_completion_queue* cq, const char* method, size_t method_len,
    const char* host, size_t host_len, gpr_timespec deadline) {
  grpc_slice method_slice = grpc_slice_from_copied_buffer(method, method_len);
  grpc_slice* host_slice_ptr = nullptr;
  grpc_slice host_slice;
  if (host != nullptr) {
    host_slice = grpc_slice_from_copied_buffer(host, host_len);
    host_slice_ptr = &host_slice;
  } else {
    // to silent msvc false warning
    host_slice = grpc_empty_slice();
  }
  grpc_call* ret =
      grpc_channel_create_call(channel, parent_call, propagation_mask, cq,
                               method_slice, host_slice_ptr, deadline, nullptr);
  grpc_slice_unref(method_slice);
  if (host != nullptr) {
    grpc_slice_unref(host_slice);
  }
  return ret;
}

/* Channel args */

GPR_EXPORT grpc_channel_args* GPR_CALLTYPE
grpcwrap_channel_args_create(size_t num_args) {
  auto* args = (grpc_channel_args*)gpr_malloc(sizeof(grpc_channel_args));
  memset(args, 0, sizeof(grpc_channel_args));

  args->num_args = num_args;
  args->args = (grpc_arg*)gpr_malloc(sizeof(grpc_arg) * num_args);
  memset(args->args, 0, sizeof(grpc_arg) * num_args);
  return args;
}

GPR_EXPORT void GPR_CALLTYPE grpcwrap_channel_args_set_string(
    grpc_channel_args* args, size_t index, const char* key, const char* value) {
  GPR_ASSERT(args);
  GPR_ASSERT(index < args->num_args);
  args->args[index].type = GRPC_ARG_STRING;
  args->args[index].key = gpr_strdup(key);
  args->args[index].value.string = gpr_strdup(value);
}

GPR_EXPORT void GPR_CALLTYPE grpcwrap_channel_args_set_integer(
    grpc_channel_args* args, size_t index, const char* key, int value) {
  GPR_ASSERT(args);
  GPR_ASSERT(index < args->num_args);
  args->args[index].type = GRPC_ARG_INTEGER;
  args->args[index].key = gpr_strdup(key);
  args->args[index].value.integer = value;
}

GPR_EXPORT void GPR_CALLTYPE grpcwrap_channel_args_set_pointer_vtable(
    grpc_channel_args* args, size_t index, const char* key, void* value,
    const grpc_arg_pointer_vtable* vtable) {
  GPR_ASSERT(args);
  GPR_ASSERT(index < args->num_args);
  args->args[index].type = GRPC_ARG_POINTER;
  args->args[index].key = gpr_strdup(key);
  args->args[index].value.pointer.p = vtable->copy(value);
  args->args[index].value.pointer.vtable = vtable;
}

GPR_EXPORT void GPR_CALLTYPE
grpcwrap_channel_args_destroy(grpc_channel_args* args) {
  size_t i;
  if (args) {
    for (i = 0; i < args->num_args; i++) {
      gpr_free(args->args[i].key);
      if (args->args[i].type == GRPC_ARG_STRING) {
        gpr_free(args->args[i].value.string);
      }
      if (args->args[i].type == GRPC_ARG_POINTER) {
        args->args[i].value.pointer.vtable->destroy(
            args->args[i].value.pointer.p);
      }
    }
    gpr_free(args->args);
    gpr_free(args);
  }
}

/* Call */

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_start_unary(
    grpc_call* call, grpcwrap_batch_context* ctx, grpc_slice* send_buffer,
    uint32_t write_flags, grpc_metadata_array* initial_metadata,
    uint32_t initial_metadata_flags, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[6];
  memset(ops, 0, sizeof(ops));
  ops[0].op = GRPC_OP_SEND_INITIAL_METADATA;
  grpcwrap_metadata_array_move(&(ctx->send_initial_metadata), initial_metadata);
  ops[0].data.send_initial_metadata.count = ctx->send_initial_metadata.count;
  ops[0].data.send_initial_metadata.metadata =
      ctx->send_initial_metadata.metadata;
  ops[0].flags = initial_metadata_flags;
  ops[0].reserved = nullptr;

  ops[1].op = GRPC_OP_SEND_MESSAGE;
  ctx->send_message = grpc_raw_byte_buffer_create(send_buffer, 1);
  ops[1].data.send_message.send_message = ctx->send_message;
  ops[1].flags = write_flags;
  ops[1].reserved = nullptr;

  ops[2].op = GRPC_OP_SEND_CLOSE_FROM_CLIENT;
  ops[2].flags = 0;
  ops[2].reserved = nullptr;

  ops[3].op = GRPC_OP_RECV_INITIAL_METADATA;
  ops[3].data.recv_initial_metadata.recv_initial_metadata =
      &(ctx->recv_initial_metadata);
  ops[3].flags = 0;
  ops[3].reserved = nullptr;

  ops[4].op = GRPC_OP_RECV_MESSAGE;
  ops[4].data.recv_message.recv_message = &(ctx->recv_message);
  ops[4].flags = 0;
  ops[4].reserved = nullptr;

  ops[5].op = GRPC_OP_RECV_STATUS_ON_CLIENT;
  ops[5].data.recv_status_on_client.trailing_metadata =
      &(ctx->recv_status_on_client.trailing_metadata);
  ops[5].data.recv_status_on_client.status =
      &(ctx->recv_status_on_client.status);
  ops[5].data.recv_status_on_client.status_details =
      &(ctx->recv_status_on_client.status_details);
  ops[5].flags = 0;
  ops[5].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_start_client_streaming(
    grpc_call* call, grpcwrap_batch_context* ctx,
    grpc_metadata_array* initial_metadata, uint32_t initial_metadata_flags,
    void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[4];
  memset(ops, 0, sizeof(ops));
  ops[0].op = GRPC_OP_SEND_INITIAL_METADATA;
  grpcwrap_metadata_array_move(&(ctx->send_initial_metadata), initial_metadata);
  ops[0].data.send_initial_metadata.count = ctx->send_initial_metadata.count;
  ops[0].data.send_initial_metadata.metadata =
      ctx->send_initial_metadata.metadata;
  ops[0].flags = initial_metadata_flags;
  ops[0].reserved = nullptr;

  ops[1].op = GRPC_OP_RECV_INITIAL_METADATA;
  ops[1].data.recv_initial_metadata.recv_initial_metadata =
      &(ctx->recv_initial_metadata);
  ops[1].flags = 0;
  ops[1].reserved = nullptr;

  ops[2].op = GRPC_OP_RECV_MESSAGE;
  ops[2].data.recv_message.recv_message = &(ctx->recv_message);
  ops[2].flags = 0;
  ops[2].reserved = nullptr;

  ops[3].op = GRPC_OP_RECV_STATUS_ON_CLIENT;
  ops[3].data.recv_status_on_client.trailing_metadata =
      &(ctx->recv_status_on_client.trailing_metadata);
  ops[3].data.recv_status_on_client.status =
      &(ctx->recv_status_on_client.status);
  ops[3].data.recv_status_on_client.status_details =
      &(ctx->recv_status_on_client.status_details);
  ops[3].flags = 0;
  ops[3].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_start_server_streaming(
    grpc_call* call, grpcwrap_batch_context* ctx, grpc_slice* send_buffer,
    uint32_t write_flags, grpc_metadata_array* initial_metadata,
    uint32_t initial_metadata_flags, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[4];
  memset(ops, 0, sizeof(ops));
  ops[0].op = GRPC_OP_SEND_INITIAL_METADATA;
  grpcwrap_metadata_array_move(&(ctx->send_initial_metadata), initial_metadata);
  ops[0].data.send_initial_metadata.count = ctx->send_initial_metadata.count;
  ops[0].data.send_initial_metadata.metadata =
      ctx->send_initial_metadata.metadata;
  ops[0].flags = initial_metadata_flags;
  ops[0].reserved = nullptr;

  ops[1].op = GRPC_OP_SEND_MESSAGE;
  ctx->send_message = grpc_raw_byte_buffer_create(send_buffer, 1);
  ops[1].data.send_message.send_message = ctx->send_message;
  ops[1].flags = write_flags;
  ops[1].reserved = nullptr;

  ops[2].op = GRPC_OP_SEND_CLOSE_FROM_CLIENT;
  ops[2].flags = 0;
  ops[2].reserved = nullptr;

  ops[3].op = GRPC_OP_RECV_STATUS_ON_CLIENT;
  ops[3].data.recv_status_on_client.trailing_metadata =
      &(ctx->recv_status_on_client.trailing_metadata);
  ops[3].data.recv_status_on_client.status =
      &(ctx->recv_status_on_client.status);
  ops[3].data.recv_status_on_client.status_details =
      &(ctx->recv_status_on_client.status_details);
  ops[3].flags = 0;
  ops[3].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_start_duplex_streaming(
    grpc_call* call, grpcwrap_batch_context* ctx,
    grpc_metadata_array* initial_metadata, uint32_t initial_metadata_flags,
    void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[2];
  memset(ops, 0, sizeof(ops));
  ops[0].op = GRPC_OP_SEND_INITIAL_METADATA;
  grpcwrap_metadata_array_move(&(ctx->send_initial_metadata), initial_metadata);
  ops[0].data.send_initial_metadata.count = ctx->send_initial_metadata.count;
  ops[0].data.send_initial_metadata.metadata =
      ctx->send_initial_metadata.metadata;
  ops[0].flags = initial_metadata_flags;
  ops[0].reserved = nullptr;

  ops[1].op = GRPC_OP_RECV_STATUS_ON_CLIENT;
  ops[1].data.recv_status_on_client.trailing_metadata =
      &(ctx->recv_status_on_client.trailing_metadata);
  ops[1].data.recv_status_on_client.status =
      &(ctx->recv_status_on_client.status);
  ops[1].data.recv_status_on_client.status_details =
      &(ctx->recv_status_on_client.status_details);
  ops[1].flags = 0;
  ops[1].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_recv_initial_metadata(
    grpc_call* call, grpcwrap_batch_context* ctx, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[1];
  ops[0].op = GRPC_OP_RECV_INITIAL_METADATA;
  ops[0].data.recv_initial_metadata.recv_initial_metadata =
      &(ctx->recv_initial_metadata);
  ops[0].flags = 0;
  ops[0].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_send_message(
    grpc_call* call, grpcwrap_batch_context* ctx, grpc_slice* send_buffer,
    uint32_t write_flags, int32_t send_empty_initial_metadata, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[2];
  memset(ops, 0, sizeof(ops));
  size_t nops = send_empty_initial_metadata ? 2 : 1;
  ops[0].op = GRPC_OP_SEND_MESSAGE;
  ctx->send_message = grpc_raw_byte_buffer_create(send_buffer, 1);
  ops[0].data.send_message.send_message = ctx->send_message;
  ops[0].flags = write_flags;
  ops[0].reserved = nullptr;
  ops[1].op = GRPC_OP_SEND_INITIAL_METADATA;
  ops[1].flags = 0;
  ops[1].reserved = nullptr;

  return grpc_call_start_batch(call, ops, nops, tag, nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE
grpcwrap_call_send_close_from_client(grpc_call* call, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[1];
  ops[0].op = GRPC_OP_SEND_CLOSE_FROM_CLIENT;
  ops[0].flags = 0;
  ops[0].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_send_status_from_server(
    grpc_call* call, grpcwrap_batch_context* ctx, grpc_status_code status_code,
    const char* status_details, size_t status_details_len,
    grpc_metadata_array* initial_metadata, grpc_metadata_array* trailing_metadata,
    int32_t send_empty_initial_metadata, grpc_slice* optional_send_buffer,
    uint32_t write_flags, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[3];
  memset(ops, 0, sizeof(ops));
  size_t nops = 1;
  grpc_slice status_details_slice =
      grpc_slice_from_copied_buffer(status_details, status_details_len);
  ops[0].op = GRPC_OP_SEND_STATUS_FROM_SERVER;
  ops[0].data.send_status_from_server.status = status_code;
  ops[0].data.send_status_from_server.status_details = &status_details_slice;
  grpcwrap_metadata_array_move(
      &(ctx->send_status_from_server.trailing_metadata), trailing_metadata);
  ops[0].data.send_status_from_server.trailing_metadata_count =
      ctx->send_status_from_server.trailing_metadata.count;
  ops[0].data.send_status_from_server.trailing_metadata =
      ctx->send_status_from_server.trailing_metadata.metadata;
  ops[0].flags = 0;
  ops[0].reserved = nullptr;
  if (optional_send_buffer) {
    ops[nops].op = GRPC_OP_SEND_MESSAGE;
    ctx->send_message = grpc_raw_byte_buffer_create(optional_send_buffer, 1);
    ops[nops].data.send_message.send_message = ctx->send_message;
    ops[nops].flags = write_flags;
    ops[nops].reserved = nullptr;
    nops++;
  }
  if (send_empty_initial_metadata) {
    ops[nops].op = GRPC_OP_SEND_INITIAL_METADATA;
    ops[nops].flags = 0;
    ops[nops].reserved = nullptr;
    nops++;
  } else if (initial_metadata) {
    ops[nops].op = GRPC_OP_SEND_INITIAL_METADATA;
    grpcwrap_metadata_array_move(&(ctx->send_initial_metadata), initial_metadata);
    ops[nops].data.send_initial_metadata.count = ctx->send_initial_metadata.count;
    ops[nops].data.send_initial_metadata.metadata = ctx->send_initial_metadata.metadata;
    ops[nops].flags = 0;
    ops[nops].reserved = nullptr;
    nops++;
  }
  grpc_call_error ret = grpc_call_start_batch(call, ops, nops, tag, nullptr);
  grpc_slice_unref(status_details_slice);
  return ret;
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_recv_message(
    grpc_call* call, grpcwrap_batch_context* ctx, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[1];
  ops[0].op = GRPC_OP_RECV_MESSAGE;
  ops[0].data.recv_message.recv_message = &(ctx->recv_message);
  ops[0].flags = 0;
  ops[0].reserved = nullptr;
  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_start_serverside(
    grpc_call* call, grpcwrap_batch_context* ctx, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[1];
  ops[0].op = GRPC_OP_RECV_CLOSE_ON_SERVER;
  ops[0].data.recv_close_on_server.cancelled =
      (&ctx->recv_close_on_server_cancelled);
  ops[0].flags = 0;
  ops[0].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

GPR_EXPORT grpc_call_error GPR_CALLTYPE grpcwrap_call_send_initial_metadata(
    grpc_call* call, grpcwrap_batch_context* ctx,
    grpc_metadata_array* initial_metadata, void* tag) {
  /* TODO: don't use magic number */
  grpc_op ops[1];
  memset(ops, 0, sizeof(ops));
  ops[0].op = GRPC_OP_SEND_INITIAL_METADATA;
  grpcwrap_metadata_array_move(&(ctx->send_initial_metadata), initial_metadata);
  ops[0].data.send_initial_metadata.count = ctx->send_initial_metadata.count;
  ops[0].data.send_initial_metadata.metadata =
      ctx->send_initial_metadata.metadata;
  ops[0].flags = 0;
  ops[0].reserved = nullptr;

  return grpc_call_start_batch(call, ops, sizeof(ops) / sizeof(ops[0]), tag,
                               nullptr);
}

/** Kick call's completion queue, it should be called after there is an event
    ready to poll.
    THREAD SAFETY: grpcwrap_call_kick_completion_queue is thread-safe
    because it does not change the call's state. */
GPR_EXPORT grpc_call_error GPR_CALLTYPE
grpcwrap_call_kick_completion_queue(grpc_call* call, void* tag) {
  // Empty batch grpc_op kicks call's completion queue immediately.
  return grpc_call_start_batch(call, nullptr, 0, tag, nullptr);
}

/* Server */

GPR_EXPORT grpc_call_error GPR_CALLTYPE
grpcwrap_server_request_call(grpc_server* server, grpc_completion_queue* cq,
                             grpcwrap_request_call_context* ctx, void* tag) {
  return grpc_server_request_call(server, &(ctx->call), &(ctx->call_details),
                                  &(ctx->request_metadata), cq, cq, tag);
}
