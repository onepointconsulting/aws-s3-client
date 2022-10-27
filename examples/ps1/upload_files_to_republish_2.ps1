cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder1
cd examples\ps1