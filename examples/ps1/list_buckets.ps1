
cd ..\..
cargo build
$env:AWS_ACCESS_KEY_ID='AKIARXNITSYZLQG3MRLY'
target\debug\aws_client.exe --mode list-buckets --region eu-central-1
cd examples\ps1