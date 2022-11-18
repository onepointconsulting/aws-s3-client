
cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode move-multiple --bucket mdm-eu-prod-republish -l ^.*folder3.+ --target-key folder3_relocation
cd examples\ps1