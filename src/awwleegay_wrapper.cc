#include <nan.h>
#include "platform.h"

extern "C"
{
    char* rust_hello();
    void rust_capitalize(char *);
    int rust_add(int, int);
}


void Hello(const Nan::FunctionCallbackInfo<v8::Value> &info)
{

    char* str = rust_hello();
    rust_capitalize(str);
    using namespace std;
    info.GetReturnValue().Set(Nan::New(str).ToLocalChecked());

    free(str);//FIXME: need to run free() by Rust
}

void Add(const Nan::FunctionCallbackInfo<v8::Value> &info)
{
    v8::Local<v8::Context> context = info.GetIsolate()->GetCurrentContext();

    if (info.Length() < 2)
    {
        Nan::ThrowTypeError("Wrong number of arguments");
        return;
    }

    if (!info[0]->IsNumber() || !info[1]->IsNumber())
    {
        Nan::ThrowTypeError("Wrong arguments");
        return;
    }

    double arg0 = info[0]->NumberValue(context).FromJust();
    double arg1 = info[1]->NumberValue(context).FromJust();
    v8::Local<v8::Number> num = Nan::New(rust_add(arg0, arg1));

    info.GetReturnValue().Set(num);
}

#define export_native_to_node(native_method, node_method) \
    (void)exports->Set(                                   \
        context,                                          \
        Nan::New(node_method).ToLocalChecked(),           \
        Nan::New<v8::FunctionTemplate>(native_method)->GetFunction(context).ToLocalChecked());

void Init(v8::Local<v8::Object> exports)
{
    v8::Local<v8::Context> context = exports->CreationContext();

    export_native_to_node(Hello, "hello");
    export_native_to_node(Add, "add");
}

NODE_MODULE(hello, Init)
