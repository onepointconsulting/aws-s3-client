cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode copy-single --bucket mdm-eu-prod-republish --source-key folder3/test.txt --target-key folder3/test_copy.txt
cd examples\ps1