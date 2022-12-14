# AWS S3 Client

## Introduction

Simple binary programme list AWS files with regular expressions that can perform the following operations on AWS S3:

- List files using regular expression filters
- Upload files with flatten or recursive mode
- Download files with flatten or recursive mode
- Delete files
- Copy multiple files to target folder
- Move multiple files to target folder
- Copy single file
- Move single file
- List buckets
- Create single bucket
- Delete single bucket
- Copy single file from one bucket to another

This library requires that `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` are accessible in some form. 

## Examples

Please check the [examples](examples) folder in this project.

## Build

```ps1
cargo build -r
```

## Integration Tests

```ps1
cargo test -- --color always --nocapture
```

Please note that for Windows you will need rc.exe from the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/) in your classpath 
so that you can successfully build the executable.

## Instructions

```
Simple binary programme list AWS files with regular expressions and also upload and download files
to and from AWS S3.

Example 1: aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-dev-drools
--list-regex-pattern "^.*be.+jar$"

Example 2: aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g
data\*.txt --target-folder folder_test_gil

USAGE:
    aws_client.exe [OPTIONS] --region <REGION> --mode <MODE>

OPTIONS:
    -a, --asc <ASC>
            Used to sort either in ascending or descending order for all operations that list files
            on S3

    -b, --bucket <BUCKET>
            The bucket in S3

    -f, --flatten
            Used to upload files to a flat or otherwise recursive structure

    -g, --glob-pattern <*>
            The glob pattern used to list files, e.g. *.zip or /media/**/*.csv to be uploaded

    -h, --help
            Print help information

    -l, --list-regex-pattern <.+>
            The regex pattern used to filter list files, e.g. .+\.zip

    -m, --mode <MODE>
            The operation mode
            
            [possible values: list, upload, download, delete, copy-single, move-single,
            copy-multiple, move-multiple, list-buckets, create-bucket, delete-bucket]

    -r, --region <REGION>
            The AWS region, like 'us-west-2', 'eu-central-1', 'eu-west-1', 'eu-west-2'

    -s, --sep <,>
            The separator used by the default printer

        --source-key <SOURCE_KEY>
            Source key for copy or move operations

        --strict-bucket
            Used to filter buckets strictly

    -t, --target-folder <TARGET_FOLDER>
            The key prefix in S3 (something like the target folder) This is also the target folder
            for download

        --target-key <TARGET_KEY>
            Target key for copy or move operations

```

## Usage Examples

- List Folder

```powershell
aws_client.exe --region eu-central-1 --mode list --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_test_gil.+
```

- Upload files

```powershell
aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_test_gil --flatten
```

- Delete remote files

```powershell
aws_client.exe --region eu-central-1 --mode delete --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_test_gil.+
```

- Copy multiple files

```powershell
aws_client.exe --region eu-central-1 --mode copy-multiple --bucket mdm-eu-prod-republish -l ^.*folder3.+ --target-key folder3_copy
```

- List Buckets

```powershell
target\debug\aws_client.exe --mode list-buckets --region eu-central-1
```