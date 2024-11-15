// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'line_chart.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Output {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) file,
    required TResult Function(Uint8List field0) memory,
    required TResult Function() none,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? file,
    TResult? Function(Uint8List field0)? memory,
    TResult? Function()? none,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? file,
    TResult Function(Uint8List field0)? memory,
    TResult Function()? none,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Output_File value) file,
    required TResult Function(Output_Memory value) memory,
    required TResult Function(Output_None value) none,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Output_File value)? file,
    TResult? Function(Output_Memory value)? memory,
    TResult? Function(Output_None value)? none,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Output_File value)? file,
    TResult Function(Output_Memory value)? memory,
    TResult Function(Output_None value)? none,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OutputCopyWith<$Res> {
  factory $OutputCopyWith(Output value, $Res Function(Output) then) =
      _$OutputCopyWithImpl<$Res, Output>;
}

/// @nodoc
class _$OutputCopyWithImpl<$Res, $Val extends Output>
    implements $OutputCopyWith<$Res> {
  _$OutputCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$Output_FileImplCopyWith<$Res> {
  factory _$$Output_FileImplCopyWith(
          _$Output_FileImpl value, $Res Function(_$Output_FileImpl) then) =
      __$$Output_FileImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Output_FileImplCopyWithImpl<$Res>
    extends _$OutputCopyWithImpl<$Res, _$Output_FileImpl>
    implements _$$Output_FileImplCopyWith<$Res> {
  __$$Output_FileImplCopyWithImpl(
      _$Output_FileImpl _value, $Res Function(_$Output_FileImpl) _then)
      : super(_value, _then);

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Output_FileImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Output_FileImpl extends Output_File {
  const _$Output_FileImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Output.file(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Output_FileImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Output_FileImplCopyWith<_$Output_FileImpl> get copyWith =>
      __$$Output_FileImplCopyWithImpl<_$Output_FileImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) file,
    required TResult Function(Uint8List field0) memory,
    required TResult Function() none,
  }) {
    return file(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? file,
    TResult? Function(Uint8List field0)? memory,
    TResult? Function()? none,
  }) {
    return file?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? file,
    TResult Function(Uint8List field0)? memory,
    TResult Function()? none,
    required TResult orElse(),
  }) {
    if (file != null) {
      return file(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Output_File value) file,
    required TResult Function(Output_Memory value) memory,
    required TResult Function(Output_None value) none,
  }) {
    return file(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Output_File value)? file,
    TResult? Function(Output_Memory value)? memory,
    TResult? Function(Output_None value)? none,
  }) {
    return file?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Output_File value)? file,
    TResult Function(Output_Memory value)? memory,
    TResult Function(Output_None value)? none,
    required TResult orElse(),
  }) {
    if (file != null) {
      return file(this);
    }
    return orElse();
  }
}

abstract class Output_File extends Output {
  const factory Output_File(final String field0) = _$Output_FileImpl;
  const Output_File._() : super._();

  String get field0;

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Output_FileImplCopyWith<_$Output_FileImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Output_MemoryImplCopyWith<$Res> {
  factory _$$Output_MemoryImplCopyWith(
          _$Output_MemoryImpl value, $Res Function(_$Output_MemoryImpl) then) =
      __$$Output_MemoryImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$Output_MemoryImplCopyWithImpl<$Res>
    extends _$OutputCopyWithImpl<$Res, _$Output_MemoryImpl>
    implements _$$Output_MemoryImplCopyWith<$Res> {
  __$$Output_MemoryImplCopyWithImpl(
      _$Output_MemoryImpl _value, $Res Function(_$Output_MemoryImpl) _then)
      : super(_value, _then);

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Output_MemoryImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$Output_MemoryImpl extends Output_Memory {
  const _$Output_MemoryImpl(this.field0) : super._();

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'Output.memory(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Output_MemoryImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Output_MemoryImplCopyWith<_$Output_MemoryImpl> get copyWith =>
      __$$Output_MemoryImplCopyWithImpl<_$Output_MemoryImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) file,
    required TResult Function(Uint8List field0) memory,
    required TResult Function() none,
  }) {
    return memory(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? file,
    TResult? Function(Uint8List field0)? memory,
    TResult? Function()? none,
  }) {
    return memory?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? file,
    TResult Function(Uint8List field0)? memory,
    TResult Function()? none,
    required TResult orElse(),
  }) {
    if (memory != null) {
      return memory(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Output_File value) file,
    required TResult Function(Output_Memory value) memory,
    required TResult Function(Output_None value) none,
  }) {
    return memory(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Output_File value)? file,
    TResult? Function(Output_Memory value)? memory,
    TResult? Function(Output_None value)? none,
  }) {
    return memory?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Output_File value)? file,
    TResult Function(Output_Memory value)? memory,
    TResult Function(Output_None value)? none,
    required TResult orElse(),
  }) {
    if (memory != null) {
      return memory(this);
    }
    return orElse();
  }
}

abstract class Output_Memory extends Output {
  const factory Output_Memory(final Uint8List field0) = _$Output_MemoryImpl;
  const Output_Memory._() : super._();

  Uint8List get field0;

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Output_MemoryImplCopyWith<_$Output_MemoryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Output_NoneImplCopyWith<$Res> {
  factory _$$Output_NoneImplCopyWith(
          _$Output_NoneImpl value, $Res Function(_$Output_NoneImpl) then) =
      __$$Output_NoneImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Output_NoneImplCopyWithImpl<$Res>
    extends _$OutputCopyWithImpl<$Res, _$Output_NoneImpl>
    implements _$$Output_NoneImplCopyWith<$Res> {
  __$$Output_NoneImplCopyWithImpl(
      _$Output_NoneImpl _value, $Res Function(_$Output_NoneImpl) _then)
      : super(_value, _then);

  /// Create a copy of Output
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$Output_NoneImpl extends Output_None {
  const _$Output_NoneImpl() : super._();

  @override
  String toString() {
    return 'Output.none()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Output_NoneImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) file,
    required TResult Function(Uint8List field0) memory,
    required TResult Function() none,
  }) {
    return none();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? file,
    TResult? Function(Uint8List field0)? memory,
    TResult? Function()? none,
  }) {
    return none?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? file,
    TResult Function(Uint8List field0)? memory,
    TResult Function()? none,
    required TResult orElse(),
  }) {
    if (none != null) {
      return none();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Output_File value) file,
    required TResult Function(Output_Memory value) memory,
    required TResult Function(Output_None value) none,
  }) {
    return none(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Output_File value)? file,
    TResult? Function(Output_Memory value)? memory,
    TResult? Function(Output_None value)? none,
  }) {
    return none?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Output_File value)? file,
    TResult Function(Output_Memory value)? memory,
    TResult Function(Output_None value)? none,
    required TResult orElse(),
  }) {
    if (none != null) {
      return none(this);
    }
    return orElse();
  }
}

abstract class Output_None extends Output {
  const factory Output_None() = _$Output_NoneImpl;
  const Output_None._() : super._();
}
