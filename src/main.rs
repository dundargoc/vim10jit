#![warn(clippy::pedantic)]

use std::path::Path;

use anyhow::{bail, ensure, Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(short, long)]
    outdir: String,
}

fn _main(args: &Args) -> Result<()> {
    let filepath = Path::new(&args.file);
    ensure!(filepath.is_file(), "not a valid file");

    let contents = std::fs::read_to_string(filepath)?;
    let generated = gen::generate(
        &contents,
        gen::ParserOpts {
            mode: gen::ParserMode::Autoload {
                // TODO: Calculate this correctly...
                prefix: "ccomplete".to_string(),
            },
        },
    )
    .unwrap_or_else(|err| {
        println!("  FAILED: {}", err.1);
        err.0
    });

    match std::fs::metadata(&args.outdir) {
        Ok(m) if m.is_dir() => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => std::fs::create_dir(&args.outdir)?,

        Ok(_) => bail!("Can't create directory: a file with the same name already exists!"),
        Err(e) => Err(e).context("Failed to check out_dir")?,
    }

    let file_name = filepath.file_name().unwrap();
    let generated_file = Path::new(&args.outdir)
        .join(file_name)
        .with_extension("lua");
    println!("generated lua: {generated_file:?}");
    std::fs::write(generated_file, generated.lua)?;
    if !generated.vim.is_empty() {
        let generated_file = Path::new(&args.outdir)
            .join(file_name)
            .with_extension("vim");
        println!("generated vim: {generated_file:?}");
        std::fs::write(generated_file, generated.vim)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    parser::setup_trace();
    let args = Args::parse();
    _main(&args)
}

#[cfg(test)]
mod integration {
    use crate::Args;
    use crate::_main;

    fn test(name: &str) {
        let mut path = String::new();
        path.push_str("vim/runtime/");
        path.push_str(name);
        let _ = _main(&Args {
            file: path,
            outdir: "out".to_owned(),
        });
    }

    #[test]
    fn autoload_ccomplete() {
        test("autoload/ccomplete.vim");
    }

    #[test]
    #[should_panic]
    fn autoload_context() {
        test("autoload/context.vim");
    }

    #[test]
    #[should_panic]
    fn autoload_dist_ft() {
        test("autoload/dist/ft.vim");
    }

    #[test]
    #[should_panic]
    fn autoload_dist_script() {
        test("autoload/dist/script.vim");
    }

    #[test]
    #[should_panic]
    fn autoload_dist_vimindent() {
        test("autoload/dist/vimindent.vim");
    }

    #[test]
    #[should_panic]
    fn autoload_typeset() {
        test("autoload/typeset.vim");
    }

    #[test]
    #[should_panic]
    fn colors_tools_check_colors() {
        test("colors/tools/check_colors.vim");
    }

    #[test]
    #[should_panic]
    fn compiler_context() {
        test("compiler/context.vim");
    }

    #[test]
    #[should_panic]
    fn ftplugin() {
        test("ftplugin.vim");
    }

    //
    #[test]
    #[should_panic]
    fn ftplugin_context() {
        test("ftplugin/context.vim");
    }

    #[test]
    #[should_panic]
    fn ftplugin_gdscript() {
        test("ftplugin/gdscript.vim");
    }

    #[test]
    #[should_panic]
    fn ftplugin_gdshader() {
        test("ftplugin/gdshader.vim");
    }

    //
    #[test]
    #[should_panic]
    fn ftplugin_mf() {
        test("ftplugin/mf.vim");
    }

    // ftplugin_mp
    #[test]
    #[should_panic]
    fn ftplugin_mp() {
        test("ftplugin/mp.vim");
    }

    #[test]
    fn ftplugof() {
        test("ftplugof.vim");
    }

    #[test]
    fn import_dist_vimhelp() {
        test("import/dist/vimhelp.vim");
    }

    #[test]
    #[should_panic]
    fn import_dist_vimhighlight() {
        test("import/dist/vimhighlight.vim");
    }

    #[test]
    fn indent_context() {
        test("indent/context.vim");
    }

    #[test]
    fn indent_gdscript() {
        test("indent/gdscript.vim");
    }

    #[test]
    #[should_panic]
    fn indent_mp() {
        test("indent/mp.vim");
    }

    #[test]
    #[should_panic]
    fn indent_vim() {
        test("indent/vim.vim");
    }

    #[test]
    #[should_panic]
    fn makemenu() {
        test("makemenu.vim");
    }

    #[test]
    fn pack_dist_opt_cfilter_plugin_cfilter() {
        test("pack/dist/opt/cfilter/plugin/cfilter.vim");
    }

    #[test]
    #[should_panic]
    fn syntax_context() {
        test("syntax/context.vim");
    }

    #[test]
    #[should_panic]
    fn syntax_mf() {
        test("syntax/mf.vim");
    }

    #[test]
    #[should_panic]
    fn syntax_mp() {
        test("syntax/mp.vim");
    }

    #[test]
    fn syntax_shared_context_data_context() {
        test("syntax/shared/context-data-context.vim");
    }

    #[test]
    fn syntax_shared_context_data_interfaces() {
        test("syntax/shared/context-data-interfaces.vim");
    }

    #[test]
    fn syntax_shared_context_data_metafun() {
        test("syntax/shared/context-data-metafun.vim");
    }

    #[test]
    fn syntax_shared_context_data_tex() {
        test("syntax/shared/context-data-tex.vim");
    }
}
