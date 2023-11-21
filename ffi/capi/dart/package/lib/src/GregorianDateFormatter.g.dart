// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X TypedDateFormatter object capable of formatting a [`IsoDateTime`] as a string,
/// using the Gregorian Calendar.
///
/// See the [Rust documentation for `TypedDateFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html) for more information.
class GregorianDateFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  GregorianDateFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XGregorianDateFormatter_destroy'));

  /// Creates a new [`GregorianDateFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new_with_length`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.try_new_with_length) for more information.
  ///
  /// Throws [Error] on failure.
  factory GregorianDateFormatter.withLength(
      DataProvider provider, Locale locale, DateLength length) {
    final result = _ICU4XGregorianDateFormatter_create_with_length(
        provider._underlying, locale._underlying, length.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return GregorianDateFormatter._(result.union.ok);
  }
  // ignore: non_constant_identifier_names
  static final _ICU4XGregorianDateFormatter_create_with_length = _capi<
          ffi.NativeFunction<
              _ResultOpaqueInt32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Int32)>>('ICU4XGregorianDateFormatter_create_with_length')
      .asFunction<
          _ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Formats a [`IsoDate`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatIsoDate(IsoDate value) {
    final writeable = _Writeable();
    final result = _ICU4XGregorianDateFormatter_format_iso_date(
        _underlying, value._underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XGregorianDateFormatter_format_iso_date = _capi<
              ffi.NativeFunction<
                  _ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGregorianDateFormatter_format_iso_date')
      .asFunction<
          _ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`IsoDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatIsoDatetime(IsoDateTime value) {
    final writeable = _Writeable();
    final result = _ICU4XGregorianDateFormatter_format_iso_datetime(
        _underlying, value._underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XGregorianDateFormatter_format_iso_datetime = _capi<
              ffi.NativeFunction<
                  _ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGregorianDateFormatter_format_iso_datetime')
      .asFunction<
          _ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
