#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/// A group that associates JavaScript contexts with one another.
/// See also https://developer.apple.com/documentation/javascriptcore/jscontextgroupref
pub type JSContextGroupRef = *const OpaqueJSContextGroup;
/// A JavaScript execution context.
/// See also https://developer.apple.com/documentation/javascriptcore/jscontextref
pub type JSContextRef = *const OpaqueJSContext;
/// A global JavaScript execution context.
/// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextref
pub type JSGlobalContextRef = *const OpaqueJSContext;
/// A UTF-16 character buffer.
/// See also https://developer.apple.com/documentation/javascriptcore/jsstringref
pub type JSStringRef = *const OpaqueJSString;
/// A JavaScript class.
/// See also https://developer.apple.com/documentation/javascriptcore/jsclassref
pub type JSClassRef = *const OpaqueClass;
/// A JavaScript value.
/// See also https://developer.apple.com/documentation/javascriptcore/jsvalueref
pub type JSValueRef = *const OpaqueJSValue;
/// A JavaScript object.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectref
pub type JSObjectRef = *const OpaqueJSValue;
/// An ordered set of the names of a JavaScript object’s properties.
/// See also https://developer.apple.com/documentation/javascriptcore/jspropertynameaccumulatorref
pub type JSPropertyNameAccumulatorRef = *const OpaqueJSPropertyNameAccumulator;
/// An array of JavaScript property names.
/// See also https://developer.apple.com/documentation/javascriptcore/jspropertynamearrayref
pub type JSPropertyNameArrayRef = *const OpaqueJSPropertyNameArray;

/// A Unicode character.
/// See also https://developer.apple.com/documentation/javascriptcore/jschar
pub type JSChar = u16;

/// Constants that identify the type of a JavaScript value.
/// See also https://developer.apple.com/documentation/javascriptcore/jstype
pub type JSType = std::ffi::c_uint;
/// The unique undefined value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypeundefined
pub const kJSTypeUndefined: JSType = 0;
/// The unique null value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypenull
pub const kJSTypeNull: JSType = 1;
/// A primitive boolean value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypeboolean
pub const kJSTypeBoolean: JSType = 2;
/// A primitive number value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypenumber
pub const kJSTypeNumber: JSType = 3;
/// A primitive string value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypestring
pub const kJSTypeString: JSType = 4;
/// An object value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypeobject
pub const kJSTypeObject: JSType = 5;
/// A primitive symbol value.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypesymbol
pub const kJSTypeSymbol: JSType = 6;
/// A bigint value. Since macOS 15
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypebigint
pub const kJSTypeBigInt: JSType = 7;

/// The type of a JavaScript typed array object.
/// See also https://developer.apple.com/documentation/javascriptcore/jstypedarraytype
pub type JSTypedArrayType = std::ffi::c_uint;
/// Not a typed array.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypenone
pub const kJSTypedArrayTypeInt8Array: JSTypedArrayType = 0;
/// A 16-bit integer array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypeint16array
pub const kJSTypedArrayTypeInt16Array: JSTypedArrayType = 1;
/// A 32-bit integer array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypeint32array
pub const kJSTypedArrayTypeInt32Array: JSTypedArrayType = 2;
/// An 8-bit unsigned integer array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypeuint8array
pub const kJSTypedArrayTypeUint8Array: JSTypedArrayType = 3;
/// An 8-bit unsigned integer clamped array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypeuint8clampedarray
pub const kJSTypedArrayTypeUint8ClampedArray: JSTypedArrayType = 4;
/// A 16-bit unsigned integer array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypeuint16array
pub const kJSTypedArrayTypeUint16Array: JSTypedArrayType = 5;
/// A 32-bit unsigned integer array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypeuint32array
pub const kJSTypedArrayTypeUint32Array: JSTypedArrayType = 6;
/// A 32-bit floating point array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypefloat32array
pub const kJSTypedArrayTypeFloat32Array: JSTypedArrayType = 7;
/// A 64-bit floating point array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypefloat64array
pub const kJSTypedArrayTypeFloat64Array: JSTypedArrayType = 8;
/// An array buffer type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypearraybuffer
pub const kJSTypedArrayTypeArrayBuffer: JSTypedArrayType = 9;
/// Not a typed array.
/// https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypenone
pub const kJSTypedArrayTypeNone: JSTypedArrayType = 10;
/// A 64-bit signed bigint array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypebigint64array
pub const kJSTypedArrayTypeBigInt64Array: JSTypedArrayType = 11;
/// A 64-bit unsigned biguint array type.
/// See also https://developer.apple.com/documentation/javascriptcore/kjstypedarraytypebiguint64array
pub const kJSTypedArrayTypeBigUint64Array: JSTypedArrayType = 12;

/// A set of JavaScript property attributes.
/// See also https://developer.apple.com/documentation/javascriptcore/jspropertyattributes
pub type JSPropertyAttributes = std::ffi::c_uint;
/// An attribute that specifies that a property has no special attributes.
/// See also https://developer.apple.com/documentation/javascriptcore/kjspropertyattributenone
pub const kJSPropertyAttributeNone: JSPropertyAttributes = 0;
/// An attribute that specifies that a property is read-only.
/// See also https://developer.apple.com/documentation/javascriptcore/kjspropertyattributereadonly
pub const kJSPropertyAttributeReadOnly: JSPropertyAttributes = 2;
/// An attribute that specifies that property enumerators and JavaScript for-in loops don’t enumerate a property.
/// See also https://developer.apple.com/documentation/javascriptcore/kjspropertyattributedontenum
pub const kJSPropertyAttributeDontEnum: JSPropertyAttributes = 4;
/// An attribute that specifies that the delete operation fails on a property.
/// See also https://developer.apple.com/documentation/javascriptcore/kjspropertyattributedontdelete
pub const kJSPropertyAttributeDontDelete: JSPropertyAttributes = 8;

/// A set of JavaScript class attributes.
/// See also https://developer.apple.com/documentation/javascriptcore/jsclassattributes
pub type JSClassAttributes = std::ffi::c_uint;
/// An attribute that specifies that a class has no special attributes.
/// See also https://developer.apple.com/documentation/javascriptcore/kjsclassattributenone
pub const kJSClassAttributeNone: JSClassAttributes = 0;
/// An attribute that specifies that a class doesn’t automatically generate a shared prototype for its instance objects.
/// See also https://developer.apple.com/documentation/javascriptcore/kjsclassattributenoautomaticprototype
pub const kJSClassAttributeNoAutomaticPrototype: JSClassAttributes = 2;

/// The callback type for using an object as a constructor.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectcallasconstructorcallback
pub type JSObjectCallAsContructorCallback = extern "C" fn(
  context: JSContextRef,
  constructor: JSObjectRef,
  args_count: std::ffi::c_int,
  args: *const JSValueRef,
  exception: *mut JSValueRef,
) -> JSObjectRef;
/// The callback type for calling an object as a function.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectcallasfunctioncallback
pub type JSObjectCallAsFunctionCallback = extern "C" fn(
  context: JSContextRef,
  function: JSObjectRef,
  this_object: JSObjectRef,
  args_count: std::ffi::c_int,
  args: *const JSValueRef,
  exception: *mut JSValueRef,
) -> JSValueRef;
/// The callback type for getting a property’s value.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetpropertycallback
pub type JSObjectGetPropertyCallback = extern "C" fn(
  context: JSContextRef,
  object: JSObjectRef,
  property_name: JSStringRef,
  exception: *mut JSValueRef,
) -> JSValueRef;
/// The callback type for setting a property’s value.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectsetpropertycallback
pub type JSObjectSetPropertyCallback = extern "C" fn(
  context: JSContextRef,
  object: JSObjectRef,
  property_name: JSStringRef,
  value: JSValueRef,
  exception: *mut JSValueRef,
) -> bool;
/// The callback type for determining whether an object has a property.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjecthaspropertycallback
pub type JSObjectHasPropertyCallback =
  extern "C" fn(context: JSContextRef, object: JSObjectRef, property_name: JSStringRef) -> bool;
/// The callback type for deleting a property.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectdeletepropertycallback
pub type JSObjectDeletePropertyCallback = extern "C" fn(
  context: JSContextRef,
  object: JSObjectRef,
  property_name: JSStringRef,
  exception: *mut JSValueRef,
) -> bool;
/// The callback type for collecting the names of an object’s properties.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetpropertynamescallback
pub type JSObjectGetPropertyNamesCallback =
  extern "C" fn(context: JSContextRef, object: JSObjectRef, accumulator: JSPropertyNameAccumulatorRef);
/// The callback type for checking whether an object is an instance of a particular type.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjecthasinstancecallback
pub type JSObjectHasInstanceCallback = extern "C" fn(
  context: JSContextRef,
  constructor: JSObjectRef,
  possible_instance: JSValueRef,
  exception: *mut JSValueRef,
) -> bool;
/// The callback type for converting an object to a particular JavaScript type.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectconverttotypecallback
pub type JSObjectConvertToTypeCallback =
  extern "C" fn(context: JSContextRef, object: JSObjectRef, r#type: JSType, exception: *mut JSValueRef) -> JSValueRef;
/// The callback type for first creating an object.
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectinitializecallback
pub type JSObjectInitializeCallback = extern "C" fn(context: JSContextRef, object: JSObjectRef);
/// The callback type for finalizing an object (preparing it for garbage collection).
/// See also https://developer.apple.com/documentation/javascriptcore/jsobjectfinalizecallback
pub type JSObjectFinalizeCallback = extern "C" fn(object: JSObjectRef);

/// A structure that contains properties and callbacks that define a type of object.
/// See also https://developer.apple.com/documentation/javascriptcore/jsclassdefinition
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSClassDefinition {
  parent_class: JSClassRef,
  class_name: *const std::ffi::c_char,
  version: std::ffi::c_int,
  attributes: JSClassAttributes,
  static_values: *const JSStaticValue,
  static_function: *const JSStaticFunction,
  initialize: JSObjectInitializeCallback,
  finialize: JSObjectFinalizeCallback,
  has_property: JSObjectHasPropertyCallback,
  get_property: JSObjectGetPropertyCallback,
  set_property: JSObjectSetPropertyCallback,
  delete_property: JSObjectDeletePropertyCallback,
  get_property_names: JSObjectGetPropertyNamesCallback,
  call_as_function: JSObjectCallAsFunctionCallback,
  has_instance: JSObjectHasInstanceCallback,
  call_as_constructor: JSObjectCallAsContructorCallback,
  convert_to_type: JSObjectConvertToTypeCallback,
}
/// A statically declared value property.
/// See also https://developer.apple.com/documentation/javascriptcore/jsstaticvalue
#[repr(C)]
pub struct JSStaticValue {
  name: *const std::ffi::c_char,
  get_property: JSObjectGetPropertyCallback,
  set_property: JSObjectSetPropertyCallback,
  attributes: JSPropertyAttributes,
}
/// A statically declared function property.
/// See also https://developer.apple.com/documentation/javascriptcore/jsstaticfunction
#[repr(C)]
pub struct JSStaticFunction {
  name: *const std::ffi::c_char,
  call_as_function: JSObjectCallAsFunctionCallback,
  attributes: JSPropertyAttributes,
}

/// A function that deallocates bytes that pass to a typed array constructor.
/// See also https://developer.apple.com/documentation/javascriptcore/jstypedarraybytesdeallocator
pub type JSTypedArrayBytesDeallocator =
  extern "C" fn(bytes: *mut std::ffi::c_void, deallocatorContext: *mut std::ffi::c_void);

extern "C" {
  /// Gets the context group that a JavaScript execution context belongs to.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscontextgetgroup(_:)
  pub fn JSContextGetGroup(context: JSContextRef) -> JSContextGroupRef;
  /// Creates a JavaScript context group.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscontextgroupcreate()
  pub fn JSContextGroupCreate() -> JSContextGroupRef;
  /// Retains a JavaScript context group.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscontextgroupretain(_:)
  pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;
  /// Releases a JavaScript context group.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscontextgrouprelease(_:)
  pub fn JSContextGroupRelease(group: JSContextGroupRef);
  /// Gets the global context of a JavaScript execution context.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscontextgetglobalcontext(_:)
  pub fn JSContextGetGlobalContext(context: JSContextRef) -> JSGlobalContextRef;
  /// Gets the global object of a JavaScript execution context.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscontextgetglobalobject(_:)
  pub fn JSContextGetGlobalObject(context: JSContextRef) -> JSObjectRef;

  /// Creates a global JavaScript execution context.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextcreate(_:)
  pub fn JSGlobalContextCreate(global_object_class: JSClassRef) -> JSGlobalContextRef;
  /// Creates a global JavaScript execution context in the provided context group.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextcreateingroup(_:_:)
  pub fn JSGlobalContextCreateInGroup(group: JSContextGroupRef, global_object_class: JSClassRef) -> JSGlobalContextRef;
  /// Retains a global JavaScript execution context.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextretain(_:)
  pub fn JSGlobalContextRetain(context: JSGlobalContextRef) -> JSGlobalContextRef;
  /// Releases a global JavaScript execution context.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextrelease(_:)
  pub fn JSGlobalContextRelease(context: JSGlobalContextRef);
  /// Gets a copy of the name of a context.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextcopyname(_:)
  pub fn JSGlobalContextCopyName(context: JSGlobalContextRef) -> JSStringRef;
  /// Sets the remote debugging name for a context.
  /// https://developer.apple.com/documentation/javascriptcore/jsglobalcontextsetname(_:_:)
  pub fn JSGlobalContextSetName(context: JSGlobalContextRef, name: JSStringRef);
  /// Returns a Boolean value that indicates whether the JavaScript context is inspectable.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextisinspectable(_:)
  pub fn JSGlobalContextIsInspectable(context: JSGlobalContextRef) -> bool;
  /// Sets a JavaScript context to be either inspectable or not inspectable.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsglobalcontextsetinspectable(_:_:)
  pub fn JSGlobalContextSetInspectable(context: JSGlobalContextRef, inspectable: bool);

  /// Creates a JavaScript string from a buffer of Unicode characters.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringcreatewithcharacters(_:_:)
  pub fn JSStringCreateWithCharacters(chars: *const JSChar, chars_count: usize) -> JSStringRef;
  /// Creates a JavaScript string from a null-terminated UTF-8 string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringcreatewithutf8cstring(_:)
  pub fn JSStringCreateWithUTF8CString(chars: *const std::ffi::c_char) -> JSStringRef;
  /// Retains a JavaScript string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringretain(_:)
  pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;
  /// Releases a JavaScript string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringrelease(_:)
  pub fn JSStringRelease(string: JSStringRef);
  /// Returns the number of Unicode characters in a JavaScript string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringgetlength(_:)
  pub fn JSStringGetLength(string: JSStringRef) -> isize;
  /// Returns a pointer to the Unicode character buffer that serves as the backing store for a JavaScript string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringgetcharactersptr(_:)
  pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;
  /// Returns the maximum number of bytes a JavaScript string uses when you convert it into a null-terminated UTF-8 string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringgetmaximumutf8cstringsize(_:)
  pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> isize;
  /// Converts a JavaScript string into a null-terminated UTF-8 string, and copies the result into an external byte buffer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringgetutf8cstring(_:_:_:)
  pub fn JSStringGetUTF8CString(string: JSStringRef, buffer: *mut std::ffi::c_char, buffer_size: isize) -> isize;
  /// Tests whether two JavaScript strings match.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringisequal(_:_:)
  pub fn JSStringIsEqual(a: JSStringRef, b: JSStringRef) -> bool;
  /// Tests whether a JavaScript string matches a null-terminated UTF-8 string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsstringisequaltoutf8cstring(_:_:)
  pub fn JSStringIsEqualToUTF8CString(a: JSStringRef, b: *const std::ffi::c_char) -> bool;

  /// Returns a JavaScript value’s type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluegettype(_:_:)
  pub fn JSValueGetType(context: JSContextRef, value: JSValueRef) -> JSType;
  /// Tests whether a JavaScript value’s type is the undefined type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisundefined(_:_:)
  pub fn JSValueIsUndefined(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value’s type is the null type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisnull(_:_:)
  pub fn JSValueIsNull(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value’s type is the boolean type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisboolean(_:_:)
  pub fn JSValueIsBoolean(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value’s type is the number type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisnumber(_:_:)
  pub fn JSValueIsNumber(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value’s type is the string type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisstring(_:_:)
  pub fn JSValueIsString(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value’s type is the symbol type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueissymbol(_:_:)
  pub fn JSValueIsSymbol(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value’s type is the object type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisobject(_:_:)
  pub fn JSValueIsObject(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value is an object with a specified class in its class chain.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisobjectofclass(_:_:_:)
  pub fn JSValueIsObjectOfClass(context: JSContextRef, value: JSValueRef, class: JSClassRef) -> bool;
  /// Tests whether a JavaScript value is an array.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisarray(_:_:)
  pub fn JSValueIsArray(context: JSContextRef, value: JSValueRef) -> bool;
  /// Tests whether a JavaScript value is a date.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisdate(_:_:)
  pub fn JSValueIsDate(context: JSContextRef, value: JSValueRef) -> bool;
  /// Returns a JavaScript value’s typed array type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluegettypedarraytype(_:_:_:)
  pub fn JSValueGetTypedArrayType(
    context: JSContextRef,
    value: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSTypedArrayType;
  /// Creates a JavaScript value of the undefined type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakeundefined(_:)
  pub fn JSValueMakeUndefined(context: JSContextRef) -> JSValueRef;
  /// Creates a JavaScript value of the null type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakenull(_:)
  pub fn JSValueMakeNull(context: JSContextRef) -> JSValueRef;
  /// Creates a JavaScript value of the boolean type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakeboolean(_:)
  pub fn JSValueMakeBoolean(context: JSContextRef, boolean: bool) -> JSValueRef;
  /// Creates a JavaScript value of the number type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakenumber(_:)
  pub fn JSValueMakeNumber(context: JSContextRef, number: std::ffi::c_double) -> JSValueRef;
  /// Creates a JavaScript value of the string type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakestring(_:)
  pub fn JSValueMakeString(context: JSContextRef, string: JSStringRef) -> JSValueRef;
  /// Creates a JavaScript value of the symbol type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakesymbol(_:)
  pub fn JSValueMakeSymbol(context: JSContextRef, description: JSStringRef) -> JSValueRef;
  /// Converts a JavaScript value to a Boolean and returns the resulting Boolean.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetoboolean(_:_:)
  pub fn JSValueToBoolean(context: JSContextRef, value: JSValueRef) -> bool;
  /// Converts a JavaScript value to a number and returns the resulting number.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetonumber(_:_:_:)
  pub fn JSValueToNumber(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> std::ffi::c_double;
  /// Converts a JavaScript value to a string and copies the result into a JavaScript string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetostringcopy(_:_:_:)
  pub fn JSValueToStringCopy(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> JSStringRef;
  /// Converts a JavaScript value to an object and returns the resulting object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetoobject(_:_:_:)
  pub fn JSValueToObject(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
  /// Creates a JavaScript value from a JSON-formatted string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluemakefromjsonstring(_:_:)
  pub fn JSValueMakeFromJSONString(context: JSContextRef, string: JSStringRef) -> JSValueRef;
  /// Creates a JavaScript string that contains the JSON-serialized representation of a JavaScript value.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluecreatejsonstring(_:_:_:_:)
  pub fn JSValueCreateJSONString(
    context: JSContextRef,
    value: JSValueRef,
    indent: std::ffi::c_uint,
    exception: *mut JSValueRef,
  ) -> JSStringRef;
  /// Tests whether two JavaScript values are equal.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisequal(_:_:_:_:)
  pub fn JSValueIsEqual(context: JSContextRef, a: JSValueRef, b: JSValueRef, exception: *mut JSValueRef) -> bool;
  /// Tests whether two JavaScript values are strict equal.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisstrictequal(_:_:_:)
  pub fn JSValueIsStrictEqual(context: JSContextRef, a: JSValueRef, b: JSValueRef, exception: *mut JSValueRef) -> bool;
  /// Tests whether a JavaScript value is an object that the specified constructor creates.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisinstanceofconstructor(_:_:_:_:)
  pub fn JSValueIsInstanceOfConstructor(
    context: JSContextRef,
    value: JSValueRef,
    constructor: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> bool;
  /// Protects a JavaScript value from garbage collection.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueprotect(_:_:)
  pub fn JSValueProtect(context: JSContextRef, value: JSValueRef);
  /// Unprotects a JavaScript value from garbage collection.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueunprotect(_:_:)
  pub fn JSValueUnprotect(context: JSContextRef, value: JSValueRef);

  /// Calls an object as a constructor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectcallasconstructor(_:_:_:_:_:)
  pub fn JSObjectCallAsConstructor(
    context: JSContextRef,
    object: JSObjectRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Calls an object as a function.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectcallasfunction(_:_:_:_:_:_:)
  pub fn JSObjectCallAsFunction(
    context: JSContextRef,
    object: JSObjectRef,
    this_object: JSObjectRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Gets the names of an object’s enumerable properties.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectcopypropertynames(_:_:)
  pub fn JSObjectCopyPropertyNames(context: JSContextRef, object: JSObjectRef) -> JSPropertyNameArrayRef;
  /// Deletes a property from an object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectdeleteproperty(_:_:_:_:)
  pub fn JSObjectDeleteProperty(
    context: JSContextRef,
    object: JSObjectRef,
    property_name: JSStringRef,
    exception: *mut JSValueRef,
  ) -> bool;
  /// Gets an object’s private data.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetprivate(_:)
  pub fn JSObjectGetPrivate(object: JSObjectRef) -> *mut std::ffi::c_void;
  /// Gets a property from an object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetproperty(_:_:_:_:)
  pub fn JSObjectGetProperty(
    context: JSContextRef,
    object: JSObjectRef,
    property_name: JSStringRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Gets a property from an object by numeric index.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetpropertyatindex(_:_:_:_:)
  pub fn JSObjectGetPropertyAtIndex(
    context: JSContextRef,
    object: JSObjectRef,
    property_index: std::ffi::c_int,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Gets an object’s prototype.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetprototype(_:_:)
  pub fn JSObjectGetPrototype(context: JSContextRef, object: JSObjectRef) -> JSValueRef;
  /// Tests whether an object has a specified property.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjecthasproperty(_:_:_:)
  pub fn JSObjectHasProperty(context: JSContextRef, object: JSObjectRef, property_name: JSStringRef) -> bool;
  /// Tests whether you can call an object as a constructor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectisconstructor(_:_:)
  pub fn JSObjectIsConstructor(context: JSContextRef, object: JSObjectRef) -> bool;
  /// Tests whether you can call an object as a function.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectisfunction(_:_:)
  pub fn JSObjectIsFunction(context: JSContextRef, object: JSObjectRef) -> bool;
  /// Creates a JavaScript object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmake(_:_:_:)
  pub fn JSObjectMake(context: JSContextRef, class: JSClassRef, data: *mut std::ffi::c_void) -> JSObjectRef;
  /// Creates a JavaScript array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakearray(_:_:_:_:)
  pub fn JSObjectMakeArray(
    context: JSContextRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript constructor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakeconstructor(_:_:_:)
  pub fn JSObjectMakeConstructor(
    context: JSContextRef,
    class: JSClassRef,
    constructor: JSObjectCallAsContructorCallback,
  ) -> JSObjectRef;
  /// Creates a JavaScript date object as though invoking the built-in date constructor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakedate(_:_:_:_:)
  pub fn JSObjectMakeDate(
    context: JSContextRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript error object as though invoking the built-in error constructor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakeerror(_:_:_:_:)
  pub fn JSObjectMakeError(
    context: JSContextRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a function with a specified script as its body.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakefunction(_:_:_:_:_:_:_:_:)
  pub fn JSObjectMakeFunction(
    context: JSContextRef,
    name: JSStringRef,
    parameter_count: std::ffi::c_uint,
    parameter_names: *const JSStringRef,
    body: JSStringRef,
    source_url: JSStringRef,
    lineno: std::ffi::c_int,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript function with a specified callback as its implementation.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakefunctionwithcallback(_:_:_:)
  pub fn JSObjectMakeFunctionWithCallback(
    context: JSContextRef,
    name: JSStringRef,
    function: JSObjectCallAsFunctionCallback,
  ) -> JSObjectRef;
  /// Creates a JavaScript regular expression object as though invoking the built-in regular expression constructor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakeregexp(_:_:_:_:)
  pub fn JSObjectMakeRegExp(
    context: JSContextRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Sets a pointer to private data on an object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectsetprivate(_:_:)
  pub fn JSObjectSetPrivate(object: JSObjectRef, data: *mut std::ffi::c_void) -> bool;
  /// Sets a property on an object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectsetproperty(_:_:_:_:_:_:)
  pub fn JSObjectSetProperty(
    context: JSContextRef,
    object: JSObjectRef,
    property_name: JSStringRef,
    value: JSValueRef,
    attributes: JSPropertyAttributes,
    exception: *mut JSValueRef,
  );
  /// Sets a property on an object by numeric index.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectsetpropertyatindex(_:_:_:_:_:)
  pub fn JSObjectSetPropertyAtIndex(
    context: JSContextRef,
    object: JSObjectRef,
    property_index: std::ffi::c_uint,
    value: JSValueRef,
    exception: *mut JSValueRef,
  );
  /// Gets a property from an object using a JavaScript value as the property key.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetpropertyforkey(_:_:_:_:)
  pub fn JSObjectGetPropertyForKey(
    context: JSContextRef,
    object: JSObjectRef,
    property_key: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Sets an object’s prototype.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectsetprototype(_:_:_:)
  pub fn JSObjectSetPrototype(context: JSContextRef, object: JSContextRef, value: JSValueRef);
  /// Deletes a property from an object using a JavaScript value as the property key.
  /// https://developer.apple.com/documentation/javascriptcore/jsobjectdeletepropertyforkey(_:_:_:_:)
  pub fn JSObjectDeletePropertyForKey(
    context: JSContextRef,
    object: JSObjectRef,
    property_key: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool;
  /// Tests whether an object has the specified property using a JavaScript value as the property key.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjecthaspropertyforkey(_:_:_:_:)
  pub fn JSObjectHasPropertyForKey(
    context: JSContextRef,
    object: JSObjectRef,
    property_key: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool;
  /// Sets a property on an object using a JavaScript value as the property key.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectsetpropertyforkey(_:_:_:_:_:_:)
  pub fn JSObjectSetPropertyForKey(
    context: JSContextRef,
    object: JSObjectRef,
    property_key: JSValueRef,
    value: JSValueRef,
    attributes: JSPropertyAttributes,
    exception: *mut JSValueRef,
  );
  /// Creates a JavaScript promise object by invoking the provided executor.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakedeferredpromise(_:_:_:_:)
  pub fn JSObjectMakeDeferredPromise(
    context: JSContextRef,
    resolve: *mut JSObjectRef,
    reject: *mut JSObjectRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript typed array object with the specified number of elements.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmaketypedarray(_:_:_:_:)
  pub fn JSObjectMakeTypedArray(
    context: JSContextRef,
    array_type: JSTypedArrayType,
    length: std::ffi::c_int,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript typed array object from an existing pointer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmaketypedarraywithbytesnocopy(_:_:_:_:_:_:_:)
  pub fn JSObjectMakeTypedArrayWithBytesNoCopy(
    context: JSContextRef,
    array_type: JSTypedArrayType,
    bytes: *mut std::ffi::c_void,
    bytes_length: std::ffi::c_int,
    byte_deallocator: JSTypedArrayBytesDeallocator,
    deallacator_context: *mut std::ffi::c_void,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript typed array object from an existing JavaScript array buffer object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmaketypedarraywitharraybuffer(_:_:_:_:)
  pub fn JSObjectMakeTypedArrayWithArrayBuffer(
    context: JSContextRef,
    array_type: JSTypedArrayType,
    buffer: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript typed array object from an existing JavaScript array buffer object with the specified offset and length.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmaketypedarraywitharraybufferandoffset(_:_:_:_:_:_:)
  pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(
    context: JSContextRef,
    array_type: JSTypedArrayType,
    buffer: JSObjectRef,
    bytes_offset: std::ffi::c_int,
    length: std::ffi::c_int,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Returns a temporary pointer to the backing store of a JavaScript typed array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgettypedarraybytesptr(_:_:_:)
  pub fn JSObjectGetTypedArrayBytesPtr(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> *mut std::ffi::c_void;
  /// Returns the length of a JavaScript typed array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgettypedarraylength(_:_:_:)
  pub fn JSObjectGetTypedArrayLength(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> std::ffi::c_int;
  /// Returns the byte length of a JavaScript typed array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgettypedarraybytelength(_:_:_:)
  pub fn JSObjectGetTypedArrayByteLength(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> std::ffi::c_int;
  /// Returns the byte offset of a JavaScript typed array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgettypedarraybyteoffset(_:_:_:)
  pub fn JSObjectGetTypedArrayByteOffset(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> std::ffi::c_int;
  /// Returns the JavaScript array buffer object to use as the backing of a JavaScript typed array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgettypedarraybuffer(_:_:_:)
  pub fn JSObjectGetTypedArrayBuffer(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
  /// Creates a JavaScript array buffer object from an existing pointer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectmakearraybufferwithbytesnocopy(_:_:_:_:_:_:)
  pub fn JSObjectMakeArrayBufferWithBytesNoCopy(
    context: JSContextRef,
    bytes: *mut std::ffi::c_void,
    bytes_length: std::ffi::c_int,
    bytes_deallocator: JSTypedArrayBytesDeallocator,
    deallocator_context: *mut std::ffi::c_void,
    exception: *mut JSStaticValue,
  ) -> JSObjectRef;
  /// Returns the number of bytes in a JavaScript data object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetarraybufferbytelength(_:_:_:)
  pub fn JSObjectGetArrayBufferByteLength(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> std::ffi::c_int;
  /// Returns a pointer to the data buffer that serves as the backing store for a JavaScript typed array object.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsobjectgetarraybufferbytesptr(_:_:_:)
  pub fn JSObjectGetArrayBufferBytesPtr(
    context: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> *mut std::ffi::c_void;

  /// Creates a JavaScript class.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsclasscreate(_:)
  pub fn JSClassCreate(definition: *const JSClassDefinition) -> JSClassRef;
  /// Releases a JavaScript class.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsclassrelease(_:)
  pub fn JSClassRelease(class: JSClassRef);
  /// Retains a JavaScript class.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsclassretain(_:)
  pub fn JSClassRetain(class: JSClassRef);

  /// Adds a property name to a JavaScript property name accumulator.
  /// See also https://developer.apple.com/documentation/javascriptcore/jspropertynameaccumulatoraddname(_:_:)
  pub fn JSPropertyNameAccumulatorAddName(accumulator: JSPropertyNameAccumulatorRef, property_name: JSStringRef);
  /// Gets a count of the number of items in a JavaScript property name array.
  /// See also https://developer.apple.com/documentation/javascriptcore/jspropertynamearraygetcount(_:)
  pub fn JSPropertyNameArrayGetCount(array: JSPropertyNameArrayRef) -> std::ffi::c_int;
  /// Gets a property name at a specified index in a JavaScript property name array.
  /// See also https://developer.apple.com/documentation/javascriptcore/jspropertynamearraygetnameatindex(_:_:)
  pub fn JSPropertyNameArrayGetNameAtIndex(array: JSPropertyNameArrayRef, index: std::ffi::c_int) -> JSStringRef;
  /// Releases a JavaScript property name array.
  /// See also https://developer.apple.com/documentation/javascriptcore/jspropertynamearrayrelease(_:)
  pub fn JSPropertyNameArrayRelease(array: JSPropertyNameArrayRef);
  /// Retains a JavaScript property name array.
  /// See also https://developer.apple.com/documentation/javascriptcore/jspropertynamearrayretain(_:)
  pub fn JSPropertyNameArrayRetain(array: JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;

  /// Checks for syntax errors in a string of JavaScript.
  /// See also https://developer.apple.com/documentation/javascriptcore/jscheckscriptsyntax(_:_:_:_:_:)
  pub fn JSCheckScriptSyntax(
    context: JSContextRef,
    script: JSStringRef,
    source_url: JSStringRef,
    lineno: std::ffi::c_int,
    exception: *mut JSValueRef,
  ) -> bool;
  /// Evaluates a string of JavaScript.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsevaluatescript(_:_:_:_:_:_:)
  pub fn JSEvaluateScript(
    context: JSContextRef,
    script: JSStringRef,
    this_object: JSObjectRef,
    source_url: JSStringRef,
    lineno: std::ffi::c_int,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Performs a JavaScript garbage collection.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsgarbagecollect(_:)
  pub fn JSGarbageCollect(context: JSContextRef);

  /// A class definition structure of the current version that contains null pointers and has no attributes.
  /// See also https://developer.apple.com/documentation/javascriptcore/kjsclassdefinitionempty
  pub static kJSClassDefinitionEmpty: JSClassDefinition;

  /// Creates a JavaScript BigInt with a double.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsbigintcreatewithdouble(_:_:_:)
  pub fn JSBigIntCreateWithDouble(
    context: JSContextRef,
    value: std::ffi::c_double,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Creates a JavaScript BigInt with a 64-bit signed integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsbigintcreatewithint64(_:_:_:)
  pub fn JSBigIntCreateWithInt64(
    context: JSContextRef,
    integer: std::ffi::c_longlong,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Creates a JavaScript BigInt with an integer represented in string.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsbigintcreatewithstring(_:_:_:)
  pub fn JSBigIntCreateWithString(context: JSContextRef, string: JSStringRef, exception: *mut JSValueRef)
    -> JSValueRef;
  /// Creates a JavaScript BigInt with a 64-bit unsigned integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsbigintcreatewithuint64(_:_:_:)
  pub fn JSBigIntCreateWithUInt64(
    context: JSContextRef,
    integer: std::ffi::c_ulonglong,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
  /// Tests whether a JavaScript value’s type is the bigint type.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvalueisbigint(_:_:)
  pub fn JSValueIsBigInt(context: JSContextRef, value: JSValueRef) -> bool;
  /// Converts a JSValue to a singed 32-bit integer and returns the resulting integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetoint32(_:_:_:)
  pub fn JSValueToInt32(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> std::ffi::c_int;
  /// Converts a JSValue to a singed 64-bit integer and returns the resulting integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetoint64(_:_:_:)
  pub fn JSValueToInt64(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> std::ffi::c_longlong;
  /// Converts a JSValue to an unsigned 32-bit integer and returns the resulting integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetoint64(_:_:_:)
  pub fn JSValueToUInt32(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> std::ffi::c_uint;
  /// Converts a JSValue to an unsigned 64-bit integer and returns the resulting integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluetouint64(_:_:_:)
  pub fn JSValueToUInt64(context: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
    -> std::ffi::c_ulonglong;
  /// Compares two JSValues.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluecompare(_:_:_:_:)
  pub fn JSValueCompare(
    context: JSContextRef,
    left: JSValueRef,
    right: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSRelationCondition;
  /// Compares a JSValue with a double.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluecomparedouble(_:_:_:_:)
  pub fn JSValueCompareDouble(
    context: JSContextRef,
    left: JSValueRef,
    right: std::ffi::c_double,
    exception: *mut JSValueRef,
  ) -> JSRelationCondition;
  /// Compares a JSValue with a signed 64-bit integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluecompareint64(_:_:_:_:)
  pub fn JSValueCompareInt64(
    context: JSContextRef,
    left: JSValueRef,
    right: std::ffi::c_longlong,
    exception: *mut JSValueRef,
  ) -> JSRelationCondition;
  /// Compares a JSValue with an unsigned 64-bit integer.
  /// See also https://developer.apple.com/documentation/javascriptcore/jsvaluecompareuint64(_:_:_:_:)
  pub fn JSValueCompareUInt64(
    context: JSContextRef,
    left: JSValueRef,
    right: std::ffi::c_ulonglong,
    exception: *mut JSValueRef,
  ) -> JSRelationCondition;
}

/// See also https://developer.apple.com/documentation/javascriptcore/jsrelationcondition
pub type JSRelationCondition = std::ffi::c_int;

#[repr(C)]
pub struct OpaqueClass {
  _opaque: [u8; 0],
}

#[repr(C)]
pub struct OpaqueJSContext {
  _opaque: [u8; 0],
}

#[repr(C)]
pub struct OpaqueJSContextGroup {
  _opaque: [u8; 0],
}

#[repr(C)]
pub struct OpaqueJSPropertyNameAccumulator {
  _opaque: [u8; 0],
}

#[repr(C)]
pub struct OpaqueJSPropertyNameArray {
  _opaque: [u8; 0],
}

#[repr(C)]
pub struct OpaqueJSString {
  _opaque: [u8; 0],
}

#[repr(C)]
pub struct OpaqueJSValue {
  _opaque: [u8; 0],
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_primitive_values() {
    unsafe {
      let context = JSGlobalContextCreate(std::ptr::null());
      let exception: *mut JSValueRef = std::ptr::null_mut();

      // Boolean
      let boolean_value = JSValueMakeBoolean(context, true);
      assert!(JSValueIsBoolean(context, boolean_value));
      assert_eq!(JSValueToBoolean(context, boolean_value), true);
      let boolean_value = JSValueMakeBoolean(context, false);
      assert!(JSValueIsBoolean(context, boolean_value));
      assert_eq!(JSValueToBoolean(context, boolean_value), false);

      // Number
      let number_value = JSValueMakeNumber(context, 114.514);
      assert!(JSValueIsNumber(context, number_value));
      assert_eq!(JSValueToNumber(context, number_value, exception), 114.514);
      assert!(exception.is_null());

      // String
      let string = JSStringCreateWithUTF8CString("114514\0".as_ptr() as *const i8);
      let string_value = JSValueMakeString(context, string);
      assert!(JSValueIsString(context, string_value));
      assert!(JSStringIsEqual(
        JSValueToStringCopy(context, string_value, exception),
        string
      ));
      assert!(exception.is_null());

      // Symbol
      let symbol_value = JSValueMakeSymbol(context, string);
      assert!(JSValueIsSymbol(context, symbol_value));
      assert_ne!(symbol_value, std::ptr::null());
    }
  }

  #[test]
  fn test_evalute_script() {
    unsafe {
      let context = JSGlobalContextCreate(std::ptr::null());
      let script = JSStringCreateWithUTF8CString("40+2\0".as_ptr() as *const i8);
      let this_object = std::ptr::null_mut();
      let source_url = JSStringCreateWithUTF8CString("test\0".as_ptr() as *const i8);
      let exception = std::ptr::null_mut();
      let value = JSEvaluateScript(context, script, this_object, source_url, 0, exception);
      assert!(exception.is_null());
      let number = JSValueToNumber(context, value, exception);
      assert!(exception.is_null());
      assert_eq!(number, 42.0);
    }
  }

  #[test]
  fn test_custom_function() {
    unsafe {
      let context = JSGlobalContextCreate(std::ptr::null());
      let exception = std::ptr::null_mut();
      let global_object = JSContextGetGlobalObject(context);
      let function_name = JSStringCreateWithUTF8CString("add\0".as_ptr() as *const i8);
      let callback = JSObjectMakeFunctionWithCallback(context, function_name, add);
      JSObjectSetProperty(
        context,
        global_object,
        function_name,
        callback,
        kJSPropertyAttributeNone,
        exception,
      );
      assert!(exception.is_null());
      let value = evalute_script(context, "add(40, 2)\0");
      let number = JSValueToNumber(context, value, exception);
      assert!(exception.is_null());
      assert_eq!(number, 42.0);
    }
  }

  #[no_mangle]
  extern "C" fn add(
    context: JSContextRef,
    function: JSObjectRef,
    this_object: JSObjectRef,
    args_count: std::ffi::c_int,
    args: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    unsafe {
      assert!(!function.is_null());
      assert_eq!(args_count, 2);
      assert_eq!(this_object, JSContextGetGlobalObject(context));
      let a = JSValueToNumber(context, *args.add(0), exception);
      let b = JSValueToNumber(context, *args.add(1), exception);
      JSValueMakeNumber(context, a + b)
    }
  }

  fn evalute_script(context: JSContextRef, script: &str) -> JSValueRef {
    unsafe {
      let script = JSStringCreateWithUTF8CString(script.as_ptr() as *const i8);
      let this_object = std::ptr::null_mut();
      let source_url = JSStringCreateWithUTF8CString("test\0".as_ptr() as *const i8);
      let exception = std::ptr::null_mut();
      let value = JSEvaluateScript(context, script, this_object, source_url, 0, exception);
      assert!(exception.is_null());
      value
    }
  }
}
