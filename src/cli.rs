use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Operation {
    List,
    Upload
}

/**
Simple binary programme list and upload local files to AWS S3.

Example 1: aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-dev-drools --list-regex-pattern "^.*be.+jar$"

Example 2: aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_test_gil
 */
#[derive(Parser)]
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
    #[clap(short, long)]
    pub(crate) target_folder: Option<String>,

    /// The operation mode
    #[clap(short, long, arg_enum)]
    pub(crate) mode: Operation,

    /// The separator used by the default printer
    #[clap(short, long, value_name = ",")]
    pub(crate) sep: Option<String>,

}