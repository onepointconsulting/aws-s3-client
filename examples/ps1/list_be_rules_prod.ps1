cd ..\..
cargo build
target\debug\aws_client.exe --region "eu-central-1" --mode list --bucket mdm-eu-prod-drools --list-regex-pattern "^.*be.+jar$"
cd examples\ps1