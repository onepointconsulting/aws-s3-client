# AWS S3 Client

## Introduction

Simple binary programme list AWS files with regular expressions that can perform the following operations on AWS S3:

- List files
- Upload files
- Download files
- Delete files

## Examples

Please check the [examples](examples) folder in this project.

## Build

```ps1
cargo build -r
```

## Instructions

```
aws_client 
Simple binary programme list AWS files with regular expressions and also upload local files to AWS
S3.

Example 1: aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-dev-drools
--list-regex-pattern "^.*be.+jar$"

Example 2: aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g
data\*.txt --target-folder folder_test_gil

USAGE:
    aws_client.exe [OPTIONS] --region <REGION> --bucket <BUCKET> --mode <MODE>

OPTIONS:
    -a, --asc <ASC>
            Used to sort either in ascending or descending order

    -b, --bucket <BUCKET>
            The bucket in S3

    -g, --glob-pattern <*>
            The glob pattern used to list files, e.g. *.zip or /media/**/*.csv to be uploaded

    -h, --help
            Print help information

    -l, --list-regex-pattern <.+>
            The regex pattern used to filter list files, e.g. .+\.zip

    -m, --mode <MODE>
            The operation mode

            [possible values: list, upload, download, delete]

    -r, --region <REGION>
            The AWS region, like 'us-west-2', 'eu-central-1', 'eu-west-1', 'eu-west-2'

    -s, --sep <,>
            The separator used by the default printer

    -t, --target-folder <TARGET_FOLDER>
            The key prefix in S3 (something like the target folder) This is also the target folder
            for download
```

## Usage Examples

- List Folder

```powershell
aws_client.exe --region eu-central-1 --mode list --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_test_gil.+
```

- Upload files

```powershell
aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_test_gil
```

- Delete remote files

```powershell
aws_client.exe --region eu-central-1 --mode delete --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_test_gil.+
```