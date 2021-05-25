use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "geo_polyfill", about = r#"A tool to apply h3's polyfill to GeoJSON."#)]
pub struct Args {
    #[structopt(short, default_value = "12", help="The resolution to pass to H3.")]
    pub resolution: u8,
}