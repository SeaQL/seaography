use url::ParseError;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    pub database_url: String,

    #[clap(value_parser)]
    pub crate_name: String,

    #[clap(value_parser)]
    pub destination: String,
}

#[derive(Debug)]
pub enum Error {
    Error(String),
    DiscovererError(seaography_discoverer::Error),
    SeaCodegenError(sea_orm_codegen::Error),
    IoError(std::io::Error),
    ParseError(url::ParseError)
}

impl From<seaography_discoverer::Error> for Error {
    fn from(err: seaography_discoverer::Error) -> Self {
        Self::DiscovererError(err)
    }
}

impl From<sea_orm_codegen::Error> for Error {
    fn from(err: sea_orm_codegen::Error) -> Self {
        Self::SeaCodegenError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/**
 * Most code depends from here
 * https://github.com/SeaQL/sea-orm/blob/master/sea-orm-cli/src/commands.rs
 */
pub fn parse_database_url(database_url: &String) -> Result<url::Url> {
    let url = url::Url::parse(&database_url)?;

    // Make sure we have all the required url components
    //
    // Missing scheme will have been caught by the Url::parse() call
    // above
    let url_username = url.username();
    let url_host = url.host_str();

    let is_sqlite = url.scheme() == "sqlite";

    // Skip checking if it's SQLite
    if !is_sqlite {
        // Panic on any that are missing
        if url_username.is_empty() {
            return Err(Error::Error(
                "No username was found in the database url".into(),
            ));
        }
        if url_host.is_none() {
            return Err(Error::Error("No host was found in the database url".into()));
        }
    }

    //
    // Make sure we have database name
    //
    if !is_sqlite {
        // The database name should be the first element of the path string
        //
        // Throwing an error if there is no database name since it might be
        // accepted by the database without it, while we're looking to dump
        // information from a particular database
        let database_name = url
            .path_segments()
            .ok_or(Error::Error(format!(
                "There is no database name as part of the url path: {}",
                url.as_str()
            )))?
            .next()
            .unwrap();

        // An empty string as the database name is also an error
        if database_name.is_empty() {
            return Err(Error::Error(format!(
                "There is no database name as part of the url path: {}",
                url.as_str()
            )));
        }
    }

    Ok(url)
}

pub fn generate_orm<P: AsRef<std::path::Path>>(
    path: &P,
    table_crate_stmts: Vec<seaography_discoverer::sea_schema::sea_query::TableCreateStatement>
) -> Result<()> {
    let entity_writer =
        sea_orm_codegen::EntityTransformer::transform(table_crate_stmts)?;

    let writer_output = entity_writer.generate(true, sea_orm_codegen::WithSerde::None);

    for sea_orm_codegen::OutputFile { name, content } in writer_output.files.iter() {
        let file_path = path.as_ref().join(name);
        std::fs::write(file_path, content.as_bytes())?;
    }

    Ok(())
}