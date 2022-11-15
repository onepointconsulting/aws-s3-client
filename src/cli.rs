use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Operation {
    List,
    Upload,
    Download,
    Delete,
    CopySingle,
    MoveSingle,
    CopyMultiple
}

/**
Simple binary programme list AWS files with regular expressions and also upload local files to AWS S3.

Example 1: aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-dev-drools --list-regex-pattern "^.*be.+jar$"

Example 2: aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_test_gil
 */
#[derive(Parser,Clone)]
pub(crate) struct Cli {

    /// The AWS region, like 'us-west-2', 'eu-central-1', 'eu-west-1', 'eu-west-2'
    #[clap(short, long)]
    pub(crate) region: String,

    /// The glob pattern used to list files, e.g. *.zip or /media/**/*.csv to be uploaded
    #[clap(short, long, value_name = "*")]
    pub(crate) glob_pattern: Option<String>,

    /// The regex pattern used to filter list files, e.g. .+\.zip
    #[clap(short, long, value_name = ".+")]
    pub(crate) list_regex_pattern: Option<String>,

    /// The bucket in S3
    #[clap(short, long)]
    pub(crate) bucket: String,

    /// The key prefix in S3 (something like the target folder)
    /// This is also the target folder for download
    #[clap(short, long)]
    pub(crate) target_folder: Option<String>,

    /// The operation mode
    #[clap(short, long, arg_enum)]
    pub(crate) mode: Operation,

    /// The separator used by the default printer
    #[clap(short, long, value_name = ",")]
    pub(crate) sep: Option<String>,

    /// Used to sort either in ascending or descending order.
    #[clap(short, long)]
    pub(crate) asc: Option<bool>,

    /// Source key for copy or move operations
    #[clap(long)]
    pub(crate) source_key: Option<String>,

    /// Target key for copy or move operations
    #[clap(long)]
    pub(crate) target_key: Option<String>


}