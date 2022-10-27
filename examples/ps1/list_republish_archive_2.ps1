cd ..\..
cargo build
target\debug\aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-prod-republish --list-regex-pattern "^.*archived/folder2.+"
cd examples\ps1