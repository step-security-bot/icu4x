// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Measurement Unit parser object which is capable of parsing the CLDR unit identifier
/// (e.g. `meter-per-square-second`) and get the [`MeasureUnit`].
///
/// See the [Rust documentation for `MeasureUnitParser`](https://docs.rs/icu/latest/icu/experimental/units/measureunit/struct.MeasureUnitParser.html) for more information.
final class MeasureUnitParser implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;
  // ignore: unused_field
  final core.List<Object> _aEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  MeasureUnitParser._fromFfi(this._ffi, this._selfEdge, this._aEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XMeasureUnitParser_destroy));

  /// Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding [`MeasureUnit`],
  /// if the identifier is valid.
  ///
  /// See the [Rust documentation for `parse`](https://docs.rs/icu/latest/icu/experimental/units/measureunit/struct.MeasureUnitParser.html#method.parse) for more information.
  MeasureUnit? parse(String unitId) {
    final temp = ffi2.Arena();
    final unitIdView = unitId.utf8View;
    final result = _ICU4XMeasureUnitParser_parse(_ffi, unitIdView.allocIn(temp), unitIdView.length);
    temp.releaseAll();
    return result.address == 0 ? null : MeasureUnit._fromFfi(result, []);
  }
}

@meta.ResourceIdentifier('ICU4XMeasureUnitParser_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XMeasureUnitParser_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XMeasureUnitParser_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XMeasureUnitParser_parse')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XMeasureUnitParser_parse')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XMeasureUnitParser_parse(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> unitIdData, int unitIdLength);
