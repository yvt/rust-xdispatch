extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_parts: Vec<_> = target.split('-').collect();
    let has_native_dispatch = target.ends_with("-apple-darwin");

    if has_native_dispatch {
        return;
    }

    // TODO: check if this works on Windows
    // TODO: check if this works on Linux

    let mut build = cc::Build::new();
    if target_parts[2] == "windows" {
        build
            .define("LIBPTHREAD_WORKQUEUE_EXPORTS", None)
            .define("MAKE_STATIC", None)
            .define("_USRDLL", None)
            .define("_WINDLL", None)
            .file("xdispatch/libpthread_workqueue/src/windows/manager.c")
            .file("xdispatch/libpthread_workqueue/src/windows/platform.c")
            .file("xdispatch/libpthread_workqueue/src/windows/thread_info.c")
            .file("xdispatch/libpthread_workqueue/src/windows/thread_rt.c");
    } else {
        build
            .file("xdispatch/libpthread_workqueue/src/posix/manager.c")
            .file("xdispatch/libpthread_workqueue/src/posix/thread_info.c")
            .file("xdispatch/libpthread_workqueue/src/posix/thread_rt.c");
    }
    build
        .file("xdispatch/libpthread_workqueue/src/api.c")
        .file("xdispatch/libpthread_workqueue/src/witem_cache.c")
        .include("xdispatch/libpthread_workqueue/include")
        .include("xdispatch/libpthread_workqueue")
        .compile("libpthread_workqueue.a");

    let mut build = cc::Build::new();
    if target_parts[2] == "windows" {
        build
            .define("MAKE_STATIC", None)
            .file("xdispatch/libkqueue/src/windows/platform.c")
            .file("xdispatch/libkqueue/src/windows/read.c")
            .file("xdispatch/libkqueue/src/windows/timer.c")
            .file("xdispatch/libkqueue/src/windows/user.c");
    } else {
        build
            .file("xdispatch/libkqueue/src/linux/platform.c")
            .file("xdispatch/libkqueue/src/linux/read.c")
            .file("xdispatch/libkqueue/src/linux/signal.c")
            .file("xdispatch/libkqueue/src/linux/timer.c")
            .file("xdispatch/libkqueue/src/linux/user.c")
            .file("xdispatch/libkqueue/src/linux/vnode.c")
            .file("xdispatch/libkqueue/src/linux/write.c");
    }
    build
        .file("xdispatch/libkqueue/src/common/filter.c")
        .file("xdispatch/libkqueue/src/common/kevent.c")
        .file("xdispatch/libkqueue/src/common/knote.c")
        .file("xdispatch/libkqueue/src/common/kqueue.c")
        .file("xdispatch/libkqueue/src/common/map.c")
        .include("xdispatch/libkqueue/include")
        .include("xdispatch/libkqueue/common")
        .compile("libkqueue.a");

    let mut build = cc::Build::new();
    if target_parts[2] == "windows" {
        build
            .file("xdispatch/libdispatch/platform/windows/platform.c")
            .include("xdispatch/libdispatch/platform/windows");
    } else {
        build.include("xdispatch/libdispatch/platform/posix");
    }
    build
        .file("xdispatch/libdispatch/src/apply.c")
        .file("xdispatch/libdispatch/src/benchmark.c")
        .file("xdispatch/libdispatch/src/blocks.c")
        .file("xdispatch/libdispatch/src/continuation_cache.c")
        .file("xdispatch/libdispatch/src/debug.c")
        .file("xdispatch/libdispatch/src/legacy.c")
        .file("xdispatch/libdispatch/src/object.c")
        .file("xdispatch/libdispatch/src/once.c")
        .file("xdispatch/libdispatch/src/protocolServer.c")
        .file("xdispatch/libdispatch/src/protocolUser.c")
        .file("xdispatch/libdispatch/src/queue.c")
        .file("xdispatch/libdispatch/src/queue_kevent.c")
        .file("xdispatch/libdispatch/src/semaphore.c")
        .file("xdispatch/libdispatch/src/shared_constructor.c")
        .file("xdispatch/libdispatch/src/source.c")
        .file("xdispatch/libdispatch/src/source_kevent.c")
        .file("xdispatch/libdispatch/src/time.c")
        .file("xdispatch/libdispatch/src/shims/time.c")
        .file("xdispatch/libdispatch/src/shims/tsd.c")
        .include("xdispatch/libdispatch/include")
        .compile("libdispatch.a");
}
