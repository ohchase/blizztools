use std::{
    io::{Cursor, Write},
    path::Path,
};

use binrw::BinRead;
use blizztools::{
    blte::BlockTable,
    cdn::parse_build_config,
    tact::{parse_cdn_table, parse_version_table},
    EncodingManifest, InstallManifest, Md5Hash,
};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy, Eq, PartialEq)]
enum Product {
    WowRetail,
    WowClassic,
    WowClassicEra,
}

impl Product {
    /// url safe path for this product
    fn cdn_path(&self) -> &'static str {
        match self {
            Product::WowRetail => "wow",
            Product::WowClassic => "wow_classic",
            Product::WowClassicEra => "wow_classic_era",
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Versions command to query tact for a product version
    Version(VersionArgs),
    /// Cdn command to query tact for cdns available for a product
    Cdn(CdnArgs),
    /// Command that will download a selected file from a version's install
    Download(DownloadArgs),
}

/// Get available versions for product
#[derive(Debug, Args)]
struct VersionArgs {
    product: Product,
}

/// Get available cdns for product
#[derive(Debug, Args)]
struct CdnArgs {
    product: Product,
}

#[derive(Debug, Args)]
struct DownloadArgs {
    product: Product,
    /// Destination folder for downloads
    output: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().without_time().compact().init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Version(args) => versions_command(args).await?,
        Commands::Cdn(args) => cdn_command(args).await?,
        Commands::Download(args) => download_command(args).await?,
    }
    Ok(())
}

async fn cdn_command(args: CdnArgs) -> anyhow::Result<()> {
    tracing::debug!("cdn called: {args:?}");
    let url = format!(
        "http://us.patch.battle.net:1119/{}/cdns",
        args.product.cdn_path()
    );
    let cdn_bytes = reqwest::get(url).await?.text().await?;
    let cdn_table = parse_cdn_table(&cdn_bytes)?;
    tracing::info!("{cdn_table:#?}");
    Ok(())
}

async fn versions_command(args: VersionArgs) -> anyhow::Result<()> {
    tracing::debug!("versions called: {args:?}");
    let url = format!(
        "http://us.patch.battle.net:1119/{}/versions",
        args.product.cdn_path()
    );

    let version_bytes = reqwest::get(url).await?.text().await?;
    let version_table = parse_version_table(&version_bytes)?;
    tracing::info!("{version_table:#?}");
    Ok(())
}

async fn download_command(args: DownloadArgs) -> anyhow::Result<()> {
    let url = format!(
        "http://us.patch.battle.net:1119/{}",
        &args.product.cdn_path()
    );
    let cdn_bytes = reqwest::get(format!("{url}/cdns")).await?.text().await?;
    let cdn_table = parse_cdn_table(&cdn_bytes)?;
    tracing::debug!("{cdn_table:#?}");

    let version_bytes = reqwest::get(format!("{url}/versions"))
        .await?
        .text()
        .await?;
    let version_table = parse_version_table(&version_bytes)?;
    tracing::debug!("{version_table:#?}");

    let cdn_definition = cdn_table
        .into_iter()
        .next()
        .ok_or(anyhow::anyhow!("atleast one cdn entry"))?;
    let selected_server = cdn_definition
        .servers
        .get(0)
        .ok_or(anyhow::anyhow!("atleast one server entry"))?;
    let version_definition = version_table
        .into_iter()
        .next()
        .ok_or(anyhow::anyhow!("atleast one version entry"))?;

    tracing::info!("latest version: {}", &version_definition.version_name);
    let output_dir = args
        .output
        .join(&args.product.cdn_path())
        .join(&version_definition.version_name);

    tracing::info!("output dir: {output_dir:?}");
    if !Path::new(&output_dir).exists() {
        std::fs::create_dir_all(&output_dir)?;
    }
    let selected_cdn = format!("{}/{}", selected_server, cdn_definition.path);
    tracing::info!("selected cdn: {selected_cdn}");

    let build_config_hash = version_definition.build_config;
    let build_config = download_config(&selected_cdn, &build_config_hash).await?;
    let build_config = parse_build_config(&build_config)?;
    tracing::debug!("{build_config:#?}");

    let encoding_config_hash = build_config.encoding.1;
    let table_data = download_by_ekey(&selected_cdn, &encoding_config_hash).await?;
    let encoding_table: EncodingManifest = EncodingManifest::read(&mut Cursor::new(table_data))?;

    let install_config_hash = build_config.install.1;
    let table_data = download_by_ekey(&selected_cdn, &install_config_hash).await?;
    let install_manifest = InstallManifest::read(&mut Cursor::new(table_data))?;

    for entry in install_manifest
        .entries
        .iter()
        .filter(|e| e.name.ends_with(b".exe") && e.name.starts_with(b"Wow"))
    {
        tracing::info!("beginning download of {}", entry.name);
        let data = download_by_ckey(&selected_cdn, &entry.hash, &encoding_table).await?;
        tracing::info!(
            "successfully downloaded {} size: {}",
            entry.name,
            data.len()
        );

        let path = output_dir.join(entry.name.to_string());
        let mut output_file = std::fs::File::create(path)?;
        output_file.write_all(&data)?;
    }
    Ok(())
}

async fn download_config(selected_cdn: &str, e_key: &Md5Hash) -> anyhow::Result<String> {
    let e_key = e_key.as_str();
    let file_url = format!(
        "https://{selected_cdn}/config/{}/{}/{e_key}",
        &e_key[0..2],
        &e_key[2..4]
    );
    tracing::debug!("requesting {file_url}");
    let bytes = reqwest::get(file_url).await?.text().await?;
    Ok(bytes)
}

async fn download_by_ekey(selected_cdn: &str, e_key: &Md5Hash) -> anyhow::Result<Vec<u8>> {
    let e_key = e_key.as_str();
    let file_url = format!(
        "https://{selected_cdn}/data/{}/{}/{e_key}",
        &e_key[0..2],
        &e_key[2..4]
    );
    tracing::debug!("requesting {file_url}");
    let blte_bytes = reqwest::get(file_url).await?.bytes().await?;
    let block_table: BlockTable = BlockTable::read(&mut Cursor::new(blte_bytes))?;
    let table_data = block_table.decompress()?;

    tracing::debug!(
        "successfully read client executable and decompressed to {} bytes",
        table_data.len()
    );
    Ok(table_data)
}

async fn download_by_ckey(
    selected_cdn: &str,
    c_key: &Md5Hash,
    encoding_table: &EncodingManifest,
) -> anyhow::Result<Vec<u8>> {
    let encoding_entry = encoding_table
        .ce_key_table_entries
        .iter()
        .find(|ce_entry| ce_entry.c_key == *c_key)
        .ok_or(anyhow::anyhow!("has ce table entry"))?;

    let e_key = encoding_entry
        .e_keys
        .get(0)
        .ok_or(anyhow::anyhow!("has ekey"))?;
    download_by_ekey(selected_cdn, &e_key).await
}
