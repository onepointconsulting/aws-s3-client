use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab_case")]
pub enum Operation {
    List,
    Upload,
    Download,
    Delete,
    CopySingle,
    MoveSingle,
    CopyMultiple,
    MoveMultiple,
    ListBuckets,
    CreateBucket,
    DeleteBucket,
    CopyBucketToBucket
}

/**
Simple binary programme list AWS files with regular expressions and also upload and download files to and from AWS S3.

Example 1: aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-dev-drools --list-regex-pattern "^.*be.+jar$"

Example 2: aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_test_gil
 */
#[derive(Parser,Clone)]
pub struct Cli {

    /// The AWS region, like 'us-west-2', 'eu-central-1', 'eu-west-1', 'eu-west-2'
    #[clap(short, long)]
    pub region: String,

    /// The glob pattern used to list files, e.g. *.zip or /media/**/*.csv to be uploaded
    #[clap(short, long, value_name = "*")]
    pub glob_pattern: Option<String>,

    /// The regex pattern used to filter list files, e.g. .+\.zip
    #[clap(short, long, value_name = ".+")]
    pub list_regex_pattern: Option<String>,

    /// The bucket in S3
    #[clap(short, long)]
    pub bucket: Option<String>,

    /// The target bucket in S3 for operations between buckets
    #[clap(long)]
    pub target_bucket: Option<String>,

    /// The key prefix in S3 (something like the target folder)
    /// This is also the target folder for download
    #[clap(short, long)]
    pub target_folder: Option<String>,

    /// The operation mode
    #[clap(short, long, arg_enum)]
    pub mode: Operation,

    /// The separator used by the default printer
    #[clap(short, long, value_name = ",")]
    pub sep: Option<String>,

    /// Used to sort either in ascending or descending order for all operations that list files on S3.
    #[clap(short, long)]
    pub asc: Option<bool>,

    /// Source key for copy or move operations
    #[clap(long)]
    pub source_key: Option<String>,

    /// Target key for copy or move operations
    #[clap(long)]
    pub target_key: Option<String>,

    /// Used to upload files to a flat or otherwise recursive structure.
    #[clap(long, short, action)]
    pub flatten: bool,

    /// Used to filter buckets strictly
    #[clap(long, action)]
    pub strict_bucket: bool,

}