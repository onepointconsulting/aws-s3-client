cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode move-single --bucket mdm-eu-prod-republish --source-key folder3/test_copy.txt --target-key folder3/test_copy_moved.txt
cd examples\ps1