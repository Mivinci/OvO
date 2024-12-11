#include "jsc.h"

#include "JavaScriptCore/APICast.h"
#include "JavaScriptCore/InitializeThreading.h"
#include "JavaScriptCore/JSCJSValue.h"
#include "JavaScriptCore/ThrowScope.h"

void JSC__initialize(void) { return JSC::initialize(); }

JSC__VM *JSC__VM__create(JSC::HeapType heap_type) {
  return JSC::VM::create(heap_type).ptr();
}

JSC__VM *JSC__VM__createContextGroup(JSC::HeapType heap_type) {
  return JSC::VM::createContextGroup(heap_type).ptr();
}

void JSC__VM__deref(JSC__VM *vm) { vm->deref(); }

JSC__JSValue JSC__evaluate(JSC__JSGlobalObject *global_object,
                           const JSC__SourceCode &source_code,
                           JSC__JSValue this_value) {
  NakedPtr<JSC::Exception> exception = nullptr;
  auto scope = DECLARE_THROW_SCOPE(global_object->vm());
  auto result =
      JSC::evaluate(global_object, source_code, JSC::JSValue::decode(this_value), exception);
  if (exception.get()) {
    scope.throwException(global_object, exception.get());
    return JSC::JSValue::encode(JSC::jsUndefined());
  }
  return JSC::JSValue::encode(result);
}
