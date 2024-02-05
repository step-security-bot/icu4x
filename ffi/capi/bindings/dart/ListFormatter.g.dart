// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `ListFormatter`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html) for more information.
final class ListFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ListFormatter._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XListFormatter_destroy));

  /// Construct a new ICU4XListFormatter instance for And patterns
  ///
  /// See the [Rust documentation for `try_new_and_with_length`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_and_with_length) for more information.
  ///
  /// Throws [Error] on failure.
  factory ListFormatter.andWithLength(DataProvider provider, Locale locale, ListLength length) {
    final result = _ICU4XListFormatter_create_and_with_length(provider._underlying, locale._underlying, length.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return ListFormatter._(result.union.ok, true);
  }

  /// Construct a new ICU4XListFormatter instance for And patterns
  ///
  /// See the [Rust documentation for `try_new_or_with_length`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_or_with_length) for more information.
  ///
  /// Throws [Error] on failure.
  factory ListFormatter.orWithLength(DataProvider provider, Locale locale, ListLength length) {
    final result = _ICU4XListFormatter_create_or_with_length(provider._underlying, locale._underlying, length.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return ListFormatter._(result.union.ok, true);
  }

  /// Construct a new ICU4XListFormatter instance for And patterns
  ///
  /// See the [Rust documentation for `try_new_unit_with_length`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_unit_with_length) for more information.
  ///
  /// Throws [Error] on failure.
  factory ListFormatter.unitWithLength(DataProvider provider, Locale locale, ListLength length) {
    final result = _ICU4XListFormatter_create_unit_with_length(provider._underlying, locale._underlying, length.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return ListFormatter._(result.union.ok, true);
  }

  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String format(List list) {
    final writeable = _Writeable();
    final result = _ICU4XListFormatter_format(_underlying, list._underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XListFormatter_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XListFormatter_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XListFormatter_create_and_with_length')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XListFormatter_create_and_with_length(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XListFormatter_create_or_with_length')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XListFormatter_create_or_with_length(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XListFormatter_create_unit_with_length')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XListFormatter_create_unit_with_length(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XListFormatter_format')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XListFormatter_format(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> list, ffi.Pointer<ffi.Opaque> writeable);