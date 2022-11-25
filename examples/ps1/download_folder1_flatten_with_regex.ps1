cd ..\..
cargo build
target\debug\aws_client.exe --region eu-central-1 --mode download --bucket mdm-eu-prod-republish -l ^.*folder1.+ -t c:\tmp\tui\republished --flatten
cd examples\ps1