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

    macro_rules! test {
        ($name:tt, $file:tt) => {
            #[test]
            fn $name() {
                test($file);
            }
        };
    }

    macro_rules! testpanic {
        ($name:tt, $file:tt) => {
            #[test]
            #[should_panic]
            fn $name() {
                test($file);
            }
        };
    }

    test!(autoload_ccomplete, "autoload/ccomplete.vim");
    testpanic!(autoload_context, "autoload/context.vim");
    testpanic!(autoload_dist_ft, "autoload/dist/ft.vim");
    testpanic!(autoload_dist_script, "autoload/dist/script.vim");
    testpanic!(autoload_dist_vimindent, "autoload/dist/vimindent.vim");
    testpanic!(autoload_typeset, "autoload/typeset.vim");
    testpanic!(colors_tools_check_colors, "colors/tools/check_colors.vim");
    testpanic!(compiler_context, "compiler/context.vim");
    testpanic!(ftplugin, "ftplugin.vim");
    testpanic!(ftplugin_context, "ftplugin/context.vim");
    testpanic!(ftplugin_gdscript, "ftplugin/gdscript.vim");
    testpanic!(ftplugin_gdshader, "ftplugin/gdshader.vim");
    testpanic!(ftplugin_mf, "ftplugin/mf.vim");
    testpanic!(ftplugin_mp, "ftplugin/mp.vim");
    test!(ftplugof, "ftplugof.vim");
    test!(import_dist_vimhelp, "import/dist/vimhelp.vim");
    testpanic!(import_dist_vimhighlight, "import/dist/vimhighlight.vim");
    test!(indent_context, "indent/context.vim");
    test!(indent_gdscript, "indent/gdscript.vim");
    testpanic!(indent_mp, "indent/mp.vim");
    testpanic!(indent_vim, "indent/vim.vim");
    testpanic!(makemenu, "makemenu.vim");
    test!(
        pack_dist_opt_cfilter_plugin_cfilter,
        "pack/dist/opt/cfilter/plugin/cfilter.vim"
    );
    testpanic!(syntax_context, "syntax/context.vim");
    testpanic!(syntax_mf, "syntax/mf.vim");
    testpanic!(syntax_mp, "syntax/mp.vim");
    test!(
        syntax_shared_context_data_context,
        "syntax/shared/context-data-context.vim"
    );
    test!(
        syntax_shared_context_data_interfaces,
        "syntax/shared/context-data-interfaces.vim"
    );
    test!(
        syntax_shared_context_data_metafun,
        "syntax/shared/context-data-metafun.vim"
    );
    test!(
        syntax_shared_context_data_tex,
        "syntax/shared/context-data-tex.vim"
    );
}
