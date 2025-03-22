use std::{
    collections::HashMap,
    fs, iter,
    path::{Path, PathBuf},
};

fn main() {
    let target_rust_dir = Path::new(&std::env::args().nth(1).unwrap()).to_owned();

    // Content of all the source files.
    let mut source_files = fs::read_dir(target_rust_dir.join("src"))
        .unwrap()
        .map(|r| r.unwrap())
        .map(|source_file| {
            let content = String::from_utf8(fs::read(source_file.path()).unwrap()).unwrap();
            (source_file.path().to_owned(), content)
        })
        .collect::<HashMap<PathBuf, String>>();

    // Replace `libc::` with `core::ffi::` and remove `libc` altogether.
    for (_, source_text) in &mut source_files {
        *source_text = source_text.replace("libc::", "core::ffi::");
        *source_text = source_text.replace("use ::libc;", "");
    }

    // Remove casting of numeric constants which causes signed/unsigned types issues
    for (_, source_text) in &mut source_files {
        *source_text = regex::Regex::new(r#"([0-9]+) as libc::c_int as uint[0-9]+_t"#)
            .unwrap()
            .replace_all(source_text, "$1")
            .to_string();
        *source_text = regex::Regex::new(r#"([0-9]+) as libc::c_int as libc::c_ulong"#)
            .unwrap()
            .replace_all(source_text, "$1")
            .to_string();
    }

    // Remove all `#[no_mangle]` attributes
    for (_, source_text) in &mut source_files {
        *source_text = source_text.replace("#[no_mangle]", "");
    }

    // Add some lines at the head of lib.rs.
    //#RUN sed -i '1 i\#![allow(ambiguous_glob_reexports)]' lib.rs
    //#RUN sed -i '1 i\#![allow(ambiguous_glob_imports)]' lib.rs
    // TODO:
    //#RUN sed -i '1 i\#![allow(unused_parens)]' lib.rs
    //#RUN sed -i '1 i\#![allow(unused_imports)]' lib.rs

    // One very specific fix.
    for (_, source_text) in &mut source_files {
        *source_text = source_text.replace("(8).wrapping_mul", "(8u64).wrapping_mul");
    }

    // TODO:
    /*
    # There are many structs named `l_array_*_uint*_t` which have a single field with an array.
    # These structs are redefined locally in each module. Unfortunately, when a function that returns
    # a struct like this is called from a different module, the Rust compiler will yield a compilation
    # error due to the returned struct being different from the locally-defined one, even though they
    # are identical.
    # To solve this, we prefix all references to these structs with `crate::`, which guarantees that
    # they all use the same definition.
    #RUN cd src && for f in *.rs; do sed -i 's/pub struct l_array_\([_a-zA-Z0-9]*\) {/pub struct __tmp_rename_array_\1 {/g' $f; done
    #RUN cd src && for f in *.rs; do sed -i 's/ \(l_array_[_a-zA-Z0-9]*\)/ crate::\1/g' $f; done
    #RUN cd src && for f in *.rs; do sed -i 's/pub struct __tmp_rename_array_\([_a-zA-Z0-9]*\) {/pub struct l_array_\1 {/g' $f; done
    */

    // In practice, the Rust code only ever works through raw pointers
    // We replace `&mut *some_val` with `&raw mut *some_val` (and same with `const`).
    // This has the effect of removing compilation errors due to `as *mut other_type` converting from
    // a reference of identically-named but different `other_type`.
    // It also solves some unaligned pointer grabbing errors.
    for (_, source_text) in &mut source_files {
        *source_text = source_text.replace("&*", "&raw const *");
        *source_text = source_text.replace("&mut *", "&raw mut *");
        *source_text = source_text.replace("&(*", "&raw const (*");
        *source_text = source_text.replace("&mut (*", "&raw mut (*");
    }

    // Now parse all the source files.
    let mut source_files = source_files
        .into_iter()
        .map(|(path, content)| (path, syn::parse_file(&content).unwrap()))
        .collect::<HashMap<PathBuf, syn::File>>();

    // Each Rust file starts with an `extern "C" {}` block containing some definitions.
    // We remove all of them.
    for src in source_files.values_mut() {
        src.items
            .retain(|item| !matches!(item, syn::Item::ForeignMod(_)));
    }

    // The Rust files contain a lot of function definitions that are normally exported
    // in the original source code. We remove the `extern "C"` and keep track of them.
    let mut function_definitions = Vec::new();
    for (source_file_path, source_content) in source_files.iter_mut() {
        for item_fn in source_content
            .items
            .iter_mut()
            .filter_map(|item| match item {
                syn::Item::Fn(f) => Some(f),
                _ => None,
            })
            .filter(|f| matches!(f.vis, syn::Visibility::Public(_)))
        {
            // Remove `extern "C"`.
            item_fn.sig.abi = None;

            function_definitions.push((
                source_file_path.file_stem().unwrap().to_owned(),
                item_fn.sig.ident.clone(),
            ));
        }
    }

    // Now find all expressions in the source code and perform tweaks.
    for (_, source_content) in source_files.iter_mut() {
        let mut exprs = Vec::<&mut syn::Expr>::new();
        let mut stmts = Vec::<&mut syn::Stmt>::new();
        let mut items = Vec::<&mut syn::Item>::new();

        items.extend(source_content.items.iter_mut());

        loop {
            if let Some(stmt) = stmts.pop() {
                match stmt {
                    syn::Stmt::Local(syn::Local {
                        init: Some(init), ..
                    }) => exprs.push(&mut *init.expr),
                    syn::Stmt::Local(syn::Local { init: None, .. }) => {}
                    syn::Stmt::Item(item) => items.push(item),
                    syn::Stmt::Expr(expr, _) => exprs.push(expr),
                    syn::Stmt::Macro(m) => {
                        if !m.mac.tokens.is_empty() {
                            unimplemented!()
                        }
                    }
                }
                continue;
            }

            if let Some(item) = items.pop() {
                match item {
                    syn::Item::Const(c) => exprs.push(&mut *c.expr),
                    syn::Item::Enum(_) => {}
                    syn::Item::ExternCrate(_) => {}
                    syn::Item::Fn(f) => stmts.extend(f.block.stmts.iter_mut()),
                    syn::Item::ForeignMod(_) => {}
                    syn::Item::Impl(_) => todo!(),
                    syn::Item::Macro(_) => unimplemented!(),
                    syn::Item::Mod(m) => {
                        items.extend(m.content.as_mut().into_iter().flat_map(|c| c.1.iter_mut()))
                    }
                    syn::Item::Static(s) => exprs.push(&mut s.expr),
                    syn::Item::Struct(_) => {}
                    syn::Item::Trait(t) => {
                        for i in t.items.iter_mut() {
                            match i {
                                syn::TraitItem::Fn(f) => stmts.extend(
                                    f.default
                                        .as_mut()
                                        .into_iter()
                                        .flat_map(|b| b.stmts.iter_mut()),
                                ),
                                syn::TraitItem::Const(c) => {
                                    exprs.extend(c.default.as_mut().into_iter().map(|v| &mut v.1))
                                }
                                _ => {}
                            }
                        }
                    }
                    syn::Item::TraitAlias(_) => {}
                    syn::Item::Type(_) => {}
                    syn::Item::Union(_) => {}
                    syn::Item::Use(_) => {}
                    syn::Item::Verbatim(_) => unimplemented!(),
                    _ => unimplemented!(),
                }
                continue;
            }

            if let Some(expr) = exprs.pop() {
                match expr {
                    syn::Expr::Array(e) => exprs.extend(e.elems.iter_mut()),
                    syn::Expr::Assign(e) => {
                        exprs.push(&mut *e.left);
                        exprs.push(&mut *e.right);
                    }
                    syn::Expr::Async(e) => stmts.extend(e.block.stmts.iter_mut()),
                    syn::Expr::Await(e) => exprs.push(&mut *e.base),
                    syn::Expr::Binary(e) => {
                        exprs.push(&mut *e.left);
                        exprs.push(&mut *e.right);
                    }
                    syn::Expr::Block(e) => stmts.extend(e.block.stmts.iter_mut()),
                    syn::Expr::Break(e) => exprs.extend(e.expr.as_mut().map(|e| &mut **e)),
                    syn::Expr::Call(e) => {
                        exprs.push(&mut *e.func);
                        exprs.extend(e.args.iter_mut())
                    }
                    syn::Expr::Cast(e) => {
                        exprs.push(&mut *e.expr);
                    }
                    syn::Expr::Closure(e) => {
                        exprs.push(&mut *e.body);
                    }
                    syn::Expr::Const(e) => stmts.extend(e.block.stmts.iter_mut()),
                    syn::Expr::Continue(_) => {}
                    syn::Expr::Field(e) => exprs.push(&mut *e.base),
                    syn::Expr::ForLoop(e) => {
                        exprs.push(&mut *e.expr);
                        stmts.extend(e.body.stmts.iter_mut())
                    }
                    syn::Expr::Group(e) => exprs.push(&mut *e.expr),
                    syn::Expr::If(e) => {
                        exprs.push(&mut *e.cond);
                        stmts.extend(e.then_branch.stmts.iter_mut());
                        exprs.extend(e.else_branch.as_mut().map(|(_, e)| &mut **e).into_iter());
                    }
                    syn::Expr::Index(e) => {
                        exprs.push(&mut *e.expr);
                        exprs.push(&mut *e.index);
                    }
                    syn::Expr::Infer(_) => {}
                    syn::Expr::Let(e) => exprs.push(&mut *e.expr),
                    syn::Expr::Lit(_) => {}
                    syn::Expr::Loop(e) => stmts.extend(e.body.stmts.iter_mut()),
                    syn::Expr::Macro(m) => {
                        if !m.mac.tokens.is_empty() {
                            unimplemented!()
                        }
                    }
                    syn::Expr::Match(e) => {
                        exprs.push(&mut *e.expr);
                        exprs.extend(e.arms.iter_mut().flat_map(|arm| {
                            iter::once(&mut *arm.body)
                                .chain(arm.guard.as_mut().map(|(_, e)| &mut **e).into_iter())
                        }));
                    }
                    syn::Expr::MethodCall(e) => {
                        exprs.push(&mut *e.receiver);
                        exprs.extend(e.args.iter_mut())
                    }
                    syn::Expr::Paren(e) => exprs.push(&mut *e.expr),
                    syn::Expr::Path(path) => {
                        // This is where we actually do some modifications.
                        // If referring to a function that is defined in a different module,
                        // we modify the path to point to that function.
                        if let Some(module) = function_definitions
                            .iter()
                            .find(|(_, f)| path.path.segments.iter().next().unwrap().ident == *f)
                            .map(|(f, _)| f)
                        {
                            let mut new_path = syn::punctuated::Punctuated::new();
                            new_path
                                .push(syn::PathSegment::from(syn::Ident::from(
                                    <syn::Token!(crate)>::default(),
                                )));
                            new_path.push(syn::PathSegment::from(
                                syn::parse_str::<syn::Ident>("src").unwrap(),
                            ));
                            new_path.push(syn::PathSegment::from(
                                syn::parse_str::<syn::Ident>(module.to_str().unwrap()).unwrap(),
                            ));
                            new_path.push(path.path.segments.iter().next().unwrap().clone());
                            path.path.segments = new_path;
                        }
                    }
                    syn::Expr::Range(e) => {
                        exprs.extend(e.start.as_mut().map(|e| &mut **e).into_iter());
                        exprs.extend(e.end.as_mut().map(|e| &mut **e).into_iter());
                    }
                    syn::Expr::RawAddr(e) => exprs.push(&mut *e.expr),
                    syn::Expr::Reference(e) => exprs.push(&mut *e.expr),
                    syn::Expr::Repeat(e) => {
                        exprs.push(&mut *e.expr);
                        exprs.push(&mut *e.len);
                    }
                    syn::Expr::Return(e) => {
                        exprs.extend(e.expr.as_mut().map(|e| &mut **e).into_iter())
                    }
                    syn::Expr::Struct(e) => {
                        exprs.extend(e.fields.iter_mut().map(|f| &mut f.expr));
                        exprs.extend(e.rest.as_mut().map(|e| &mut **e).into_iter());
                    }
                    syn::Expr::Try(e) => exprs.push(&mut *e.expr),
                    syn::Expr::TryBlock(e) => stmts.extend(e.block.stmts.iter_mut()),
                    syn::Expr::Tuple(e) => exprs.extend(e.elems.iter_mut()),
                    syn::Expr::Unary(e) => exprs.push(&mut *e.expr),
                    syn::Expr::Unsafe(e) => stmts.extend(e.block.stmts.iter_mut()),
                    syn::Expr::Verbatim(_) => unimplemented!(),
                    syn::Expr::While(e) => {
                        exprs.push(&mut *e.cond);
                        stmts.extend(e.body.stmts.iter_mut());
                    }
                    syn::Expr::Yield(e) => {
                        exprs.extend(e.expr.as_mut().map(|e| &mut **e).into_iter())
                    }
                    _ => unimplemented!(),
                }
                continue;
            }

            break;
        }
    }

    // Write the modifications.
    for (target_path, target_content) in source_files {
        let content = quote::ToTokens::into_token_stream(target_content).to_string();
        fs::write(target_path, &content).unwrap();
    }

    std::io::Write::write_all(
        &mut fs::File::options().append(true).open(target_rust_dir.join("lib.rs")).unwrap(),
        quote::quote! {
            unsafe fn memcpy(d: *mut core::ffi::c_void, s: *mut core::ffi::c_void, c: u64) -> *mut core::ffi::c_void { core::ptr::copy_nonoverlapping::<u8>(s.cast_const().cast(), d.cast(), c as usize); d }
            unsafe fn memset(d: *mut core::ffi::c_void, ch: core::ffi::c_int, c: u64) -> *mut core::ffi::c_void { assert!(ch <= 255); core::ptr::write_bytes::<u8>(d.cast(), ch as u8, c as usize); d }
            fn abs(v: u32) -> u32 { v }
            fn cosf(v: f32) -> f32 { v.cos() }
            fn sinf(v: f32) -> f32 { v.sin() }
            fn fabsf(v: f32) -> f32 { v.abs() }
            fn roundf(v: f32) -> f32 { v.round() }
            fn sqrtf(v: f32) -> f32 { v.sqrt() }
            fn logf(v: f32) -> f32 { v.ln() }
            fn fegetround() -> core::ffi::c_uint { 0 }
            fn __assert_fail(_assertion: *mut core::ffi::c_void, _file: *mut core::ffi::c_void, _line: core::ffi::c_uint, _function: *mut core::ffi::c_void) -> ! { panic!() }
            fn LLVMMul_uov(_: core::ffi::c_ulong, a: &mut u64, b: &mut u64, out: &mut u64) -> u8 { let (res, carry) = (*a).overflowing_mul(*b); *out = res; carry as u8 }
            unsafe fn posix_memalign(memptr: *mut core::ffi::c_void, align: u64, size: u64) -> core::ffi::c_int { libc::posix_memalign(memptr as *mut _, align as usize, size as usize) }
            use libc::free;
            unsafe fn _Znwm(size: u64) -> *mut core::ffi::c_void { libc::malloc(size as libc::size_t) }
            unsafe fn _Znam(size: u64) -> *mut core::ffi::c_void { libc::malloc(size as libc::size_t) }
            unsafe fn _ZdaPv(ptr: *mut core::ffi::c_void) { libc::free(ptr) }
            unsafe fn _ZdlPvm(ptr: *mut core::ffi::c_void, _: u64) { libc::free(ptr) }
            unsafe fn _ZSt25__throw_bad_function_callv() -> ! { panic!() }
            unsafe fn _ZSt20__throw_system_errori<T>(_: T) -> ! { panic!() }
            // __cxa_begin_catch is only ever used right before a call to terminate()
            unsafe fn __cxa_begin_catch(_: *mut core::ffi::c_void) -> *mut core::ffi::c_void { core::ptr::null_mut() }
            unsafe fn _ZSt9terminatev() -> ! { panic!() }
            unsafe fn pthread_mutex_lock(mutex: *mut core::ffi::c_void) -> core::ffi::c_uint { libc::pthread_mutex_lock(mutex as *mut _) as core::ffi::c_uint }
            unsafe fn pthread_mutex_unlock(mutex: *mut core::ffi::c_void) -> core::ffi::c_uint { libc::pthread_mutex_unlock(mutex as *mut _) as core::ffi::c_uint }
        }.to_string().as_bytes()
    ).unwrap();
}
