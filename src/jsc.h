#include "JavaScriptCore/Completion.h"
#include "JavaScriptCore/Heap.h"
#include "JavaScriptCore/JSCJSValue.h"
#include "JavaScriptCore/SourceCode.h"
#include "JavaScriptCore/VM.h"

#include "wtf/ExportMacros.h"

#undef JS_EXPORT_PRIVATE
#define JS_EXPORT_PRIVATE


using JSC__VM = JSC::VM;
using JSC__JSGlobalObject = JSC::JSGlobalObject;
using JSC__JSValue = JSC::EncodedJSValue;
using JSC__SourceCode = JSC::SourceCode;
using JSC__Exception = JSC::Exception;


extern "C" {

JS_EXPORT void JSC__initialize(void);

JS_EXPORT JSC__VM *JSC__VM__create(JSC::HeapType heap_type);
JS_EXPORT JSC__VM *JSC__VM__createContextGroup(JSC::HeapType heap_type);
JS_EXPORT void JSC__VM__deref(JSC__VM *vm);

JS_EXPORT JSC__JSValue JSC__evaluate(JSC__JSGlobalObject *global_object,
                           const JSC__SourceCode &source_code,
                           JSC__JSValue this_value);

}
