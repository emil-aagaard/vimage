use std::{
    env, error,
    fmt::{self, Display},
    fs, io,
    path::PathBuf,
    process,
};

const OUTPUT_FILE: &str = ".viewimg.html";

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len().saturating_sub(1);
    if args_len == 1 {
        let relative_path_string = args.get(1).unwrap();
        let output_file_path_buf = get_output_file_path_buf()?;
        let path_buf = get_path_buf(relative_path_string)?;
        create_html(path_buf, output_file_path_buf.clone())?;
        open_browser(output_file_path_buf)
    } else {
        Err(Error::ArgsLenInvalid(args_len))
    }
}

fn get_output_file_path_buf() -> Result<PathBuf, Error> {
    let home_path_string = env::var("HOME").map_err(|_| Error::UnableToFindHome)?;
    Ok(PathBuf::from(home_path_string).join(OUTPUT_FILE))
}

fn get_path_buf(relative_path_string: &String) -> Result<PathBuf, Error> {
    fs::canonicalize(relative_path_string)
        .map_err(|_| Error::PathInvalid(relative_path_string.clone()))
}

fn create_html(path_buf: PathBuf, output_file_path_buf: PathBuf) -> Result<(), Error> {
    let path_string = path_buf.to_string_lossy();
    let html_string = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <img id="plot" src="{path_string}" style="max-width: 100%">
            <script>
                setInterval(() => {{
                    document.getElementById('plot').src = '{path_string}?t=' + Date.now();
                }}, 500);
            </script>
        </body>
        </html>
    "#
    );
    fs::write(output_file_path_buf, html_string)
        .map_err(|error| Error::UnableToWriteToOutputFile(error))
}

fn open_browser(output_file_path_buf: PathBuf) -> Result<(), Error> {
    process::Command::new("open")
        .arg(output_file_path_buf)
        .spawn()
        .map_err(|_| Error::UnableToOpenBrowser)?;
    Ok(())
}

#[derive(Debug)]
enum Error {
    ArgsLenInvalid(usize),
    PathInvalid(String),
    UnableToFindHome,
    UnableToWriteToOutputFile(io::Error),
    UnableToOpenBrowser,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgsLenInvalid(args_len) => {
                write!(f, "Expected 1 argument, received {args_len}.")
            }
            Self::PathInvalid(relative_path_string) => {
                write!(f, "Path not found: {relative_path_string}.")
            }
            Self::UnableToFindHome => write!(f, "Unable to find the HOME environment variable."),
            Self::UnableToWriteToOutputFile(error) => {
                write!(f, "Unable to write to output file: {error}.")
            }
            Self::UnableToOpenBrowser => write!(f, "Unable to open browser."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ArgsLenInvalid(_) => None,
            Self::PathInvalid(_) => None,
            Self::UnableToFindHome => None,
            Self::UnableToWriteToOutputFile(error) => Some(error),
            Self::UnableToOpenBrowser => None,
        }
    }
}
