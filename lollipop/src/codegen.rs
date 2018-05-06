use ::{
    parse,
    lower::lower,
    ast::{self, AstNode},
};

type Result<T> = ::std::result::Result<T, ::failure::Error>;

pub fn gen(input: String) -> Result<String> {
    let file = parse(input);
    let file = ast::File::cast(file.root()).unwrap();
    let (g, diags) = lower(file);
    for d in diags {
        eprintln!("{}", d);
    }
    let g = match g {
        Some(g) => g,
        None => {
            bail!("error: failed to compile grammar")
        }
    };
    println!("lowered!");
    Ok(String::new())
}
