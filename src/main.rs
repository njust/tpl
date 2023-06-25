use tera::Tera;
use anyhow::{Result, anyhow};
use std::env;

fn main() -> Result<()> {
    let tpl_dir = env::var("TPL_DIR")?;
    let tpl_dir = format!("{}/**/*.tpl", tpl_dir);
    let tera = match Tera::new(&tpl_dir) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let mut args = env::args().into_iter();
    let tpl = args.nth(1).ok_or(anyhow!("Missing required argument"))?;
    let mut ctx = tera::Context::new();
    for arg in args {
        let mut parts = arg.split("=");
        let arg = parts.next().ok_or(anyhow!("No arg"))?;
        let val = parts.next().ok_or(anyhow!("No val"))?;
        ctx.insert(arg, val );
    }
    let res = tera.render(&format!("{}.tpl", tpl), &ctx)?;
    println!("{}", res);
    Ok(())
}
