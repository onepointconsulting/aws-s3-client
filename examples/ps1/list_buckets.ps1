
cd ..\..
cargo build
target\debug\aws_client.exe --mode list-buckets --region eu-central-1
cd examples\ps1