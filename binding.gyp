{
    "targets": [
        {
            "target_name": "addon",
            "sources": ["./src/awwleegay_wrapper.cc"],        
            "conditions":[
                ['OS=="mac"',{"variables" : {'awwleegay_lib' : 'libawwleegay.a',}}],
                ['OS=="win"',{"variables" : {'awwleegay_lib' : 'awwleegay.lib',}}],
            ],
            "libraries": [
                "<(module_root_dir)/src/awwleegay/target/debug/<(awwleegay_lib)",
            ],
            'cflags_cc': ['-Wall',],
            "include_dirs": [
                "<!(node -e \"require('nan')\")"  # 引入nan.h
            ]
        }
    ]
}
