use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
    pub strict: bool,
}

impl Config {
    pub fn parse_args<I, S>(mut iter: I) -> Result<Self, String>
    where
        I: Iterator<Item = S>,
        S: Into<String>,
    {
        // 첫 번째는 바이너리 이름.
        let _bin = iter.next();

        let mut path: Option<String> = None;
        let mut strict = false;

        while let Some(arg) = iter.next() {
            let arg = arg.into();
            match arg.as_str() {
                "-h" | "--help" => {
                    return Err(Self::usage());
                }
                "-f" | "--file" => {
                    let v = iter.next().ok_or_else(|| "--file requires a value\n".to_owned() + &Self::usage())?;
                    path = Some(v.into());
                }
                "--strict" => strict = true,
                other if other.starts_with("--file=") => {
                    let v = other.split_once('=').map(|(_, v)| v.to_string()).unwrap();
                    path = Some(v);
                }
                other if other.starts_with('-') => {
                    return Err(format!("Unknown flag: {}\n{}", other, Self::usage()));
                }
                // 위치 인자: path
                other => {
                    if path.is_none() {
                        path = Some(other.to_string());
                    } else {
                        return Err(format!("Unexpected arg: {}\n{}", other, Self::usage()));
                    }
                }
            }
        }

        let path = path.ok_or_else(|| "Missing input path\n".to_owned() + &Self::usage())?;
        Ok(Config { path, strict })
    }

    pub fn usage() -> String {
        r#"Usage:
  day08_error --file <PATH> [--strict]
  day08_error <PATH> [--strict]

Options:
  -f, --file <PATH>   Input file path
      --strict        Fail on first invalid line (default: skip invalid lines)
  -h, --help          Show this help
"#
        .to_string()
    }
}

pub fn parse_from_env() -> Result<Config, String> {
    Config::parse_args(env::args())
}

