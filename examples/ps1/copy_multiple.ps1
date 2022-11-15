
cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode copy-multiple --bucket mdm-eu-prod-republish -l ^.*folder3.+ --target-key folder3_copy
cd examples\ps1