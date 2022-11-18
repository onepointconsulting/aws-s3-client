cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_ignore --flatten

target\debug\aws_client.exe --region eu-central-1 --mode list --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_ignore.+

cd examples\ps1