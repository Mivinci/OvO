#include "JavaScriptCore/JSCJSValue.h"
#include "JavaScriptCore/JSGlobalObject.h"
#include "JavaScriptCore/SourceCode.h"
#include "JavaScriptCore/VM.h"

extern "C" {
  using JSC__VM = JSC::VM;
  using JSC__JSGlobalObject = JSC::JSGlobalObject;
  using JSC__JSValue = JSC::EncodedJSValue;
  using JSC__SourceCode = JSC::SourceCode;
  using JSC__Exception = JSC::Exception;

  void JSC__initialize();
  JSC__JSValue JSC__evaluate(JSC__JSGlobalObject *global_object,
                           const JSC__SourceCode &source_code,
                           JSC__JSValue this_value);

  JSC__VM *JSC__VM__create(JSC::HeapType heap_type);
  JSC__VM *JSC__VM__createContextGroup(JSC::HeapType heap_type);
  void JSC__VM__deref(JSC__VM *vm);
}
